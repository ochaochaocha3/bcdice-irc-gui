use std::cell::RefCell;
use std::rc::Rc;

extern crate gio;
extern crate gtk;
use gtk::prelude::*;

/// ウィジェットの集合
pub struct WidgetSet {
    /// メインウィンドウ
    pub main_window: gtk::ApplicationWindow,

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
