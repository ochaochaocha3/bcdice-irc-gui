use std::cell::RefCell;
use std::rc::Rc;

extern crate gio;
extern crate glib;
extern crate gtk;
use gio::prelude::*;
use gtk::prelude::*;

extern crate clap;

extern crate tokio;
use tokio::sync::mpsc;

extern crate tonic;

mod irc_bot;
mod signal_handler;
mod widget;

use widget::{WidgetSetRef, WidgetSetRefExt};

use bcdice_irc_proto::bc_dice_irc_service_client::BcDiceIrcServiceClient;
use bcdice_irc_proto::{StopRequest, VersionRequest};

pub mod bcdice_irc_proto {
    tonic::include_proto!("bcdice_irc_proto");
}

pub enum AppEvent {
    Start,
    StopReceiver,
    Version { bcdice_irc: String, bcdice: String },
}

/// RPCの接続先URL。
const RPC_URL: &str = "http://localhost:50051";

/// アプリケーションを実行する。
pub fn run() {
    let application = gtk::Application::new(Some("org.bcdice.bcdice-irc-gui"), Default::default())
        .expect("failed to initialize GTK application");

    let irc_bot_config = irc_bot::ConfigRef::default();
    application.connect_activate(move |app| {
        let (mut send, mut recv) = mpsc::unbounded_channel::<AppEvent>();
        send.send(AppEvent::Start).unwrap_or_default();

        let widgets = {
            let glade_src = include_str!("bcdice-irc.glade");
            WidgetSetRef::create(glade_src, app)
        };

        let status_bar_context_ids = {
            let w = widgets.borrow();
            StatusBarContextIDSet::new(&w.status_bar)
        };

        setup_actions(app, &widgets, &mut send);
        setup_accelerators(app);
        setup_menus(app);

        {
            let mut w = widgets.borrow_mut();
            put_version_number_to_version_label(&mut w.bcdice_version_label);
        }

        connect_signals(
            &widgets,
            &status_bar_context_ids,
            &irc_bot_config,
            &mut send,
        );

        gtk::idle_add(glib::clone!(@strong widgets => move || {
            let received = recv.try_recv();
            match received {
                Err(mpsc::error::TryRecvError::Closed) => return Continue(false),
                Ok(AppEvent::Start) => println!("Message receiving start!"),
                Ok(AppEvent::StopReceiver) => {
                    recv.close();
                    return Continue(false);
                }
                Ok(AppEvent::Version { bcdice_irc, bcdice }) => {
                    let w = widgets.borrow_mut();
                    w.bcdice_version_label.set_text(&format!(
                    "BCDice IRC GUI v{}, BCDice IRC v{}, BCDice v{}", clap::crate_version!(), bcdice_irc, bcdice));
                }
                _ => (),
            }

            Continue(true)
        }));

        {
            let w = widgets.borrow();
            w.main_window.show_all();
        }
    });

    application.run(&[]);
}

/// アプリケーションにアクションを登録する。
fn setup_actions(
    app: &gtk::Application,
    widgets: &WidgetSetRef,
    send: &mut mpsc::UnboundedSender<AppEvent>,
) {
    let quit_action = gio::SimpleAction::new("quit", None);
    quit_action.connect_activate(glib::clone!(@strong widgets => move |_, _| {
        let w = widgets.borrow();
        w.main_window.destroy();
    }));

    let rpc_version_action = gio::SimpleAction::new("rpc_version", None);
    rpc_version_action.connect_activate(glib::clone!(@strong send => move |_, _| {
        let send = send.clone();
        tokio::spawn(async move {
            let client = BcDiceIrcServiceClient::connect(RPC_URL).await;
            let mut c = match client {
                Err(e) => {
                    println!("RPC connection error: {}", e);
                    return;
                }
                Ok(c) => c,
            };

            let request = tonic::Request::new(VersionRequest {});
            let response = c.version(request).await;
            let r = match response {
                Err(e) => {
                    println!("Couldn't get version: {}", e);
                    return;
                }
                Ok(r) => r.into_inner(),
            };

            send.send(AppEvent::Version {
                bcdice_irc: r.bcdice_irc,
                bcdice: r.bcdice,
            }).unwrap_or_default();
        });
    }));

    app.add_action(&quit_action);
    app.add_action(&rpc_version_action);
}

/// アクセラレータを用意する。
fn setup_accelerators(app: &gtk::Application) {
    app.set_accels_for_action("app.quit", &["<Primary>Q"]);
}

/// メニューを用意する。
fn setup_menus(app: &gtk::Application) {
    let menu_builder = gtk::Builder::new_from_string(include_str!("menu.xml"));

    {
        let app_menu: gio::Menu = menu_builder
            .get_object("app_menu")
            .expect("Couldn't get app_menu");

        app.set_app_menu(Some(&app_menu));
    }

    {
        let menu_bar: gio::Menu = menu_builder
            .get_object("menu_bar")
            .expect("Couldn't get menu_bar");

        app.set_menubar(Some(&menu_bar));
    }
}

/// バージョン情報ラベルにバージョン番号を入れる。
fn put_version_number_to_version_label(label: &mut gtk::Label) {
    label.set_text(&format!("BCDice IRC GUI v{}", clap::crate_version!()));
}

/// シグナルに対応するハンドラを接続する。
fn connect_signals(
    widgets: &WidgetSetRef,
    status_bar_context_ids: &StatusBarContextIDSet,
    irc_bot_config: &irc_bot::ConfigRef,
    send: &mut mpsc::UnboundedSender<AppEvent>,
) {
    use signal_handler::*;

    macro_rules! handler_ref {
        ($e: expr) => {
            Rc::new(RefCell::new(Some($e)))
        };
    }

    let mut handler_ids: SignalHandlerIdSet = Default::default();

    handler_ids.main_window_destroy = handler_ref!(connect_main_window_destroy(&widgets, send));
    handler_ids.hostname_entry_changed =
        handler_ref!(connect_hostname_entry_changed(&widgets, &irc_bot_config));
    handler_ids.port_spin_button_value_changed = handler_ref!(
        connect_port_spin_button_value_changed(&widgets, &irc_bot_config)
    );
    handler_ids.nick_entry_changed =
        handler_ref!(connect_nick_entry_changed(&widgets, &irc_bot_config));
    handler_ids.channel_entry_changed =
        handler_ref!(connect_channel_entry_changed(&widgets, &irc_bot_config));

    handler_ids.connect_disconnect_button_clicked = handler_ref!(
        connect_connect_disconnect_button_clicked(&widgets, &irc_bot_config)
    );
}

/// ステータスバーの掲載内容の型。
#[derive(Debug)]
struct StatusBarContextIDSet {
    preset_load: u32,
    save_presets: u32,
    game_system_change: u32,
    connection: u32,
}

impl StatusBarContextIDSet {
    fn new(bar: &gtk::Statusbar) -> Self {
        Self {
            preset_load: bar.get_context_id("preset_load"),
            save_presets: bar.get_context_id("save_presets"),
            game_system_change: bar.get_context_id("game_system_change"),
            connection: bar.get_context_id("connection"),
        }
    }
}

/// RPCサーバを停止する。
pub async fn stop_rpc_server() {
    let client = BcDiceIrcServiceClient::connect(RPC_URL).await;
    let mut c = match client {
        Err(_) => {
            // RPCサーバが停止していると判断する
            return;
        }
        Ok(c) => c,
    };

    let request = tonic::Request::new(StopRequest {});
    let response = c.stop(request).await;
    match response {
        Err(e) => println!("Couldn't stop server: {}", e),
        _ => (),
    }
}
