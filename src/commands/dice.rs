use rand::Rng;
use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[description = "サイコロを振る。開始数値と終了数値をdで指定できる。"]
async fn dice(ctx: &Context, msg: &Message) -> CommandResult {

    let after_command_msg: String = msg.content.chars().skip(5).collect();

    // メッセージ中の最初の/diceを削除した文字列内にdがあれば，それを区切りにして開始数値と終了数値を取得
    let range = if after_command_msg.contains("d") {
        let mut split = after_command_msg.split("d");
        let start = split.next().unwrap().trim().parse::<i32>().unwrap();
        let end = split.next().unwrap().trim().parse::<i32>().unwrap();
        start..end+1
    } else {
        1..101
    };


    msg.channel_id
        .say(&ctx.http, format!("{}\n{}", msg.author.mention(), rand::thread_rng().gen_range(range)))
        .await?;

    Ok(())
}
