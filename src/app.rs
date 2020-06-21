use std::cell::RefCell;
use std::rc::Rc;

extern crate gio;
extern crate gtk;
use gio::prelude::*;
use gtk::prelude::*;

mod irc_bot;
mod signal_handler;

/// アプリケーションを実行する。
pub fn run() {
    let application = gtk::Application::new(Some("org.bcdice.bcdice-irc-gui"), Default::default())
        .expect("failed to initialize GTK application");

    let irc_bot_config = irc_bot::ConfigRef::default();
    application.connect_activate(move |app| {
        let widgets = {
            let glade_src = include_str!("bcdice-irc.glade");
            WidgetSetRef::create(glade_src, &app)
        };

        let status_bar_context_ids = {
            let w = widgets.borrow();
            StatusBarContextIDSet::new(&w.status_bar)
        };

        connect_signals(&widgets, &status_bar_context_ids, &irc_bot_config);

        {
            let w = widgets.borrow();
            w.main_window.show_all();
        }
    });

    application.run(&[]);
}

/// シグナルに対応するハンドラを接続する。
fn connect_signals(
    widgets: &WidgetSetRef,
    status_bar_context_ids: &StatusBarContextIDSet,
    irc_bot_config: &irc_bot::ConfigRef,
) {
    use signal_handler::*;

    macro_rules! handler_ref {
        ($e: expr) => {
            Rc::new(RefCell::new(Some($e)))
        };
    }

    let mut handler_ids: SignalHandlerIdSet = Default::default();
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

/// ウィジェットの集合
pub struct WidgetSet {
    /// メインウィンドウ
    pub main_window: gtk::Window,

    /// プリセットコンボボックス
    pub preset_combo_box: gtk::ComboBox,
    /// プリセット名エントリ
    pub preset_entry: gtk::Entry,
    /// プリセット保存ボタン
    pub preset_save_button: gtk::Button,
    /// プリセット削除ボタン
    pub preset_delete_button: gtk::Button,

    /// ホスト名エントリ
    pub hostname_entry: gtk::Entry,
    /// ポート番号スピンボタン
    pub port_spin_button: gtk::SpinButton,
    /// パスワードチェックボタン
    pub password_check_button: gtk::CheckButton,
    /// パスワードエントリ
    pub password_entry: gtk::Entry,
    /// 文字コードコンボボックス
    pub encoding_combo_box: gtk::ComboBox,
    /// ニックネームエントリ
    pub nick_entry: gtk::Entry,
    /// チャンネルエントリ
    pub channel_entry: gtk::Entry,
    /// 接続/切断ボタン
    pub connect_disconnect_button: gtk::Button,

    /// ゲームシステムコンボボックス
    pub game_system_combo_box: gtk::ComboBox,
    /// ヘルプのテキストビュー
    pub help_text_view: gtk::TextView,

    /// バージョン情報ラベル
    pub bcdice_version_label: gtk::Label,

    /// ステータスバー
    pub status_bar: gtk::Statusbar,
}

pub type WidgetSetRef = Rc<RefCell<WidgetSet>>;

/// ウィジェットの集合のトレイト。
pub trait WidgetSetRefExt {
    /// インスタンスを作成する。
    fn create(glade_src: &str, app: &gtk::Application) -> Self;
}

impl WidgetSetRefExt for WidgetSetRef {
    fn create(glade_src: &str, app: &gtk::Application) -> Self {
        let builder = gtk::Builder::new_from_string(glade_src);

        macro_rules! get_object {
            ($n: expr) => {
                builder
                    .get_object($n)
                    .expect(&format!("Couldn't get {}", $n))
            };
        }

        let widgets = WidgetSet {
            main_window: get_object!("main_window"),

            preset_combo_box: get_object!("preset_combo_box"),
            preset_entry: get_object!("preset_entry"),
            preset_save_button: get_object!("preset_save_button"),
            preset_delete_button: get_object!("preset_delete_button"),

            hostname_entry: get_object!("hostname_entry"),
            port_spin_button: get_object!("port_spin_button"),
            password_check_button: get_object!("password_check_button"),
            password_entry: get_object!("password_entry"),
            encoding_combo_box: get_object!("encoding_combo_box"),
            nick_entry: get_object!("nick_entry"),
            channel_entry: get_object!("channel_entry"),
            connect_disconnect_button: get_object!("connect_disconnect_button"),

            game_system_combo_box: get_object!("game_system_combo_box"),
            help_text_view: get_object!("help_text_view"),

            bcdice_version_label: get_object!("bcdice_version_label"),

            status_bar: get_object!("status_bar"),
        };

        widgets.main_window.set_application(Some(app));

        Rc::new(RefCell::new(widgets))
    }
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
