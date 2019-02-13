use serenity::framework::StandardFramework;

pub mod anilist;
pub mod giphy;
pub mod fortune;
pub mod meta;
pub mod nekoslife;
pub mod urban;

pub fn register(mut framework: StandardFramework) -> StandardFramework {
    framework = anilist::register(framework);
    framework = fortune::register(framework);
    framework = nekoslife::register(framework);
    framework = urban::register(framework);

    framework
}