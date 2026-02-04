use clap::Parser;
use regex::Regex;
use std::fs;
use std::io::{self, Read};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "ai-deodorant")]
#[command(about = "Remove AI-like formatting from text files")]
#[command(version)]
struct Cli {
    /// Input file path (use '-' for stdin)
    #[arg(value_name = "FILE")]
    input: PathBuf,

    /// Remove emoji characters
    #[arg(long)]
    emoji: bool,
}

fn remove_bold_markers(text: &str) -> String {
    text.replace("**", "")
}

fn remove_emojis(text: &str) -> String {
    let emoji_pattern = Regex::new(concat!(
        "[",
        "\u{1F600}-\u{1F64F}", // Emoticons
        "\u{1F300}-\u{1F5FF}", // Misc Symbols and Pictographs
        "\u{1F680}-\u{1F6FF}", // Transport and Map
        "\u{1F1E0}-\u{1F1FF}", // Flags
        "\u{2600}-\u{26FF}",   // Misc symbols
        "\u{2700}-\u{27BF}",   // Dingbats
        "\u{FE00}-\u{FE0F}",   // Variation Selectors
        "\u{1F900}-\u{1F9FF}", // Supplemental Symbols and Pictographs
        "\u{1FA00}-\u{1FA6F}", // Chess Symbols
        "\u{1FA70}-\u{1FAFF}", // Symbols and Pictographs Extended-A
        "\u{231A}-\u{231B}",   // Watch, Hourglass
        "\u{23E9}-\u{23F3}",   // Various symbols
        "\u{23F8}-\u{23FA}",   // Various symbols
        "\u{25AA}-\u{25AB}",   // Squares
        "\u{25B6}",            // Play button
        "\u{25C0}",            // Reverse button
        "\u{25FB}-\u{25FE}",   // Squares
        "\u{2614}-\u{2615}",   // Umbrella, Hot beverage
        "\u{2648}-\u{2653}",   // Zodiac
        "\u{267F}",            // Wheelchair
        "\u{2693}",            // Anchor
        "\u{26A1}",            // High voltage
        "\u{26AA}-\u{26AB}",   // Circles
        "\u{26BD}-\u{26BE}",   // Soccer, Baseball
        "\u{26C4}-\u{26C5}",   // Snowman, Sun
        "\u{26CE}",            // Ophiuchus
        "\u{26D4}",            // No entry
        "\u{26EA}",            // Church
        "\u{26F2}-\u{26F3}",   // Fountain, Golf
        "\u{26F5}",            // Sailboat
        "\u{26FA}",            // Tent
        "\u{26FD}",            // Fuel pump
        "\u{2702}",            // Scissors
        "\u{2705}",            // Check mark
        "\u{2708}-\u{270D}",   // Various
        "\u{270F}",            // Pencil
        "\u{2712}",            // Black nib
        "\u{2714}",            // Check mark
        "\u{2716}",            // X mark
        "\u{271D}",            // Cross
        "\u{2721}",            // Star of David
        "\u{2728}",            // Sparkles
        "\u{2733}-\u{2734}",   // Eight spoked asterisk
        "\u{2744}",            // Snowflake
        "\u{2747}",            // Sparkle
        "\u{274C}",            // Cross mark
        "\u{274E}",            // Cross mark
        "\u{2753}-\u{2755}",   // Question marks
        "\u{2757}",            // Exclamation mark
        "\u{2763}-\u{2764}",   // Heart exclamation, Heart
        "\u{2795}-\u{2797}",   // Plus, Minus, Division
        "\u{27A1}",            // Right arrow
        "\u{27B0}",            // Curly loop
        "\u{27BF}",            // Double curly loop
        "\u{2934}-\u{2935}",   // Arrows
        "\u{2B05}-\u{2B07}",   // Arrows
        "\u{2B1B}-\u{2B1C}",   // Squares
        "\u{2B50}",            // Star
        "\u{2B55}",            // Circle
        "\u{3030}",            // Wavy dash
        "\u{303D}",            // Part alternation mark
        "\u{3297}",            // Circled Ideograph Congratulation
        "\u{3299}",            // Circled Ideograph Secret
        "]"
    ))
    .expect("Invalid emoji regex pattern");

    emoji_pattern.replace_all(text, "").to_string()
}

fn process_text(text: &str, remove_emoji: bool) -> String {
    let result = remove_bold_markers(text);
    if remove_emoji {
        remove_emojis(&result)
    } else {
        result
    }
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();

    let content = if cli.input.to_string_lossy() == "-" {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer)?;
        buffer
    } else {
        fs::read_to_string(&cli.input)?
    };

    let result = process_text(&content, cli.emoji);
    print!("{}", result);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_bold_markers() {
        assert_eq!(remove_bold_markers("**bold** text"), "bold text");
        assert_eq!(remove_bold_markers("no markers"), "no markers");
        assert_eq!(
            remove_bold_markers("**multiple** **markers**"),
            "multiple markers"
        );
    }

    #[test]
    fn test_remove_emojis() {
        assert_eq!(remove_emojis("hello 😀 world"), "hello  world");
        assert_eq!(remove_emojis("no emoji"), "no emoji");
        assert_eq!(remove_emojis("🎉 celebration 🎊"), " celebration ");
    }

    #[test]
    fn test_process_text() {
        assert_eq!(
            process_text("**bold** 😀 text", false),
            "bold 😀 text"
        );
        assert_eq!(process_text("**bold** 😀 text", true), "bold  text");
    }
}
