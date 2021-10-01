use failure::ResultExt;
use mpris::PlayerFinder;
use serenity::client::Context;
use serenity::http::AttachmentType;
use serenity::model::channel::Message;
use serenity::framework::standard::{
    Args, CommandResult,
    macros::command
};
use serenity::utils::MessageBuilder;
use std::path::Path;

#[command]
#[description = "Bot will parse the input and output the correct full link to the repo."]
#[usage = "site user/repo"]
#[example = "github Phate6660/rsfetch"]
#[example = "gitlab ArcticTheRogue/asgl"]
#[example = "codeberg Phate6660/musinfo"]
#[example = "github phate/rsPhate-2.0"]
#[num_args(2)]
pub async fn git(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let site = args.single::<String>()?;
    let repo = args.single::<String>()?;
    let match_site = site.as_str();

    // Match for site to create message.
    let message: String = match match_site {
        "github" => MessageBuilder::new()
            .push("https://github.com/")
            .push(repo)
            .build(),
        "gitlab" => MessageBuilder::new()
            .push("https://gitlab.com/")
            .push(repo)
            .build(),
        "codeberg" => MessageBuilder::new()
            .push("https://codeberg.org/")
            .push(repo)
            .build(),
        "sourcehut" => MessageBuilder::new()
            .push("https://sr.ht/~")
            .push(repo)
            .build(),
        _ => "Could not generate a full link, please try again.".to_string(),
    };

    if let Err(why) = msg.channel_id.say(&ctx.http, &message).await {
        println!("Could not push full Git repo link because: {}", why);
    }

    Ok(())
}

async fn meta() -> (String, String, String, String, String) {
    let player_finder = PlayerFinder::new().context("Could not connect to D-Bus").unwrap();
    let player = player_finder.find_active().context("Could not find any player").unwrap();
    let player_name = player.identity();
    let metadata = player.get_metadata().context("Could not get metadata").unwrap();
    let artist = &metadata.artists().unwrap()[0];
    let album = &metadata.album_name().unwrap();
    let title = &metadata.title().unwrap();
    let cover = &metadata.art_url().unwrap();
    if player_name == "Rhythmbox" {
        let full_cover = [&cover[7..], ".jpg"].concat();
        (player_name.to_string(), 
         artist.to_string(), 
         album.to_string(), 
         title.to_string(), 
         full_cover)
    } else {
        (player_name.to_string(), 
         artist.to_string(), 
         album.to_string(), 
         title.to_string(), 
         cover.to_string())
    }
}

#[command]
#[description = "Bot will reply with pretty embed containing current music info and cover art of what Phate is listening to."]
pub async fn wipltrn(ctx: &Context, msg: &Message) -> CommandResult {
    let (player_name, artist, album, title, full_cover) = meta().await;
    let cover_with_attach = if player_name == "Rhythmbox" {
        let base_cover_vec = full_cover.split('/').collect::<Vec<&str>>();
        let base_cover = base_cover_vec[7];
        ["attachment://", base_cover].concat()
    } else {
        full_cover.to_string()
    };
    let sent_message = msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.title("What Is Phate Listening To Right Now");
            e.url("https://libre.fm/user/phate6660");
            e.fields(vec![
                ("Player", &player_name, false),
                ("Title", &title, false),
                ("Album", &album, false),
                ("Artist", &artist, false)]);
            e.image(cover_with_attach);
            e.color(serenity::utils::Color::PURPLE);
            e
        });
        if player_name == "Rhythmbox" {
            m.add_file(AttachmentType::Path(Path::new(&full_cover)));
        } else {
            m.add_file(AttachmentType::Image(&full_cover));
        }
        m
    }).await;
    if let Err(why) = sent_message {
        println!("Error sending message: {:?}", why);
    }
    Ok(())
}

#[command]
#[description = "Bot will reply with pretty embed explaining why the topic is bad."]
#[usage = "topic"]
#[example = "apple"]
#[example = "steam"]
#[example = "systemd"]
pub async fn www(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let query = args.rest();
    match query {
        "apple" => {
            msg.channel_id.send_message(&ctx.http, |m| {
                m.embed(|e| {
                    e.title("Why Phate6660 hates Apple:");
                    e.fields(vec![
                        ("They are Evil", "They are a despicable, evil, and disgusting company. I find them to be even worse than Google, and probably even Amazon. They've done some truly terrible things. Some examples: mass censorship, worker abuse (manipulation/brainwashing, sweatshops), repairing your own device is not allowed, they fully support DRM, they exploit developers.", false),
                        ("They Pretend", "They like to pretend that they are the good guys of tech. While companies like Google are extremely terrible for your privacy, at least they aren't pretending like they aren't. Apple likes to give people the illusion that you can pay for your privacy, which to put frankly, is not true at all. They still spy on you just as much, or even more than, Google does.", false),
                        ("They are Restrictive and Controlling", "They limit and control what you are allowed to do with your own device. Want to repair your Mac? Nope, can't do that. Want to install a different OS? Nope, they'll do as much as they can to stop you from doing that. The reason why I prefer Google more (not that I like them, this is more about choosing the lesser evil), is because you are allowed to do something about it. Don't want Android spying on you? Most of the time (depending on the phone brand) you can easily unlock your phone, install a custom recovery, and install a custom ROM like LineageOS without installing GApps. With iPhones, try as you may, you will never have that same amount of control that you can have on an Android device.", false),
                    ]);
                    e
                });
                m
            }).await?;
        },
        "steam" => {
            msg.channel_id.send_message(&ctx.http, |m| {
                m.embed(|e| {
                    e.title("Why Phate6660 hates Steam:");
                    e.fields(vec![
                        ("DRM", "You don't own the games you buy, you own the right to play the game off of Steam.", false),
                        ("Mistreatment of Developers", "Steam mistreats game devs and publishers (this is why you see more and more games using GOG or even their own launchers/stores).", false),
                        ("It is Forced Onto You", "Steam is *forced* onto you by various games. Imagine my surprise when I buy the Elder Scrolls anthology (as a physical collector's set complete with DISC COPIES of the game), and every game works... except for Skyrim. Skyrim requires you to use Steam. It's a shame that I saved up 50 whole dollars for shit. I can tell you Skyrim went straight into the trash, right after being broken into 600 little pieces.", false),
                        ("Privacy Violations", "I shouldn't have to explain this one right?", false),
                        ("Centralization", "Having all of your games centralized into one place is stupid, and this ties into the DRM point. If Steam were to shut down right now, I guarantee that you would lose access to at least 80% of your games.", false),
                    ]);
                    e
                });
                m
            }).await?;
        },
        "systemd" => {
            msg.channel_id.send_message(&ctx.http, |m| {
                m.embed(|e| {
                    e.title("Why Phate6660 hates SystemD:");
                    e.fields(vec![
                        ("Feature Creep", "The undeniable feature creep. While some people actually enjoy the features brought in from it (boot manager, network manager, login manager?, etc), I find them to be nothing but bloat. An init system should be just that. An init system. Not <insert an exaggerated amount of functions here>.", false),
                        ("Slow", "It is slow. Slow to shutdown, slow to boot up, etc. Here are actual timed reboots from my machine using 3 init systems. SystemD (17s), OpenRC (11s), Runit (7s). 17s vs 7s, which would you choose?", false),
                        ("Bugs and Insecurities", "Due to the feature creep, there is a larger attack service for bugs and security vulnerabilities. And there are security issues with SystemD.", false),
                        ("Devs don't Care", "This is the one that bothers me the most. It's almost as if the dev(s) are completely oblivious or at least ignorant to the feature creep and security issues. Hell, Poettering even got awarded by Red Hat for making lame excuses for not fixing important bugs.", false),
                        ("Hard Requirement", "It is a hard dependency for a large and *still growing* list of programs. Which forces users to use tools such as consolekit or elogind or eudev, or even patch the program themselves not to use systemd. This is a trivial thing when using distros like Gentoo, but I feel sincerely sorry for those using distros like Debian where it's near impossible to escape.", false),
                    ]);
                    e
                });
                m
            }).await?;
        },
        _ => {
            let message = format!("unknown query: {}", query);
            msg.reply(ctx, message).await?;
        }
    }
    Ok(())
}
