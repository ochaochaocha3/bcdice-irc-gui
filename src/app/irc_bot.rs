use std::cell::RefCell;
use std::rc::Rc;

/// IRCボットの設定の構造体。
#[derive(Debug)]
pub struct Config {
    /// 設定の名前。
    pub name: String,
    /// 接続先のホスト名。
    pub hostname: String,
    /// 接続先のポート。
    pub port: u16,
    /// 接続時にパスワードを使用するか。
    pub use_password: bool,
    /// 接続時に使用するパスワード。
    pub password: String,
    /// IRCサーバの文字エンコーディング。
    pub encoding: String,
    /// ニックネーム。
    pub nick: String,
    /// 最初にJOINするチャンネル。
    pub channel: String,
    /// ゲームシステムID。
    pub game_system_id: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            name: "Default".to_string(),
            hostname: "irc.trpg.net".to_string(),
            port: 6667,
            use_password: false,
            password: "".to_string(),
            encoding: "UTF-8".to_string(),
            nick: "BCDice".to_string(),
            channel: "Dice_Test".to_string(),
            game_system_id: "DiceBot".to_string(),
        }
    }
}

pub type ConfigRef = Rc<RefCell<Config>>;

trait ConfigRefExt {
    fn default() -> Self;
}

impl ConfigRefExt for ConfigRef {
    fn default() -> Self {
        let c: Config = Default::default();
        Rc::new(RefCell::new(c))
    }
}
