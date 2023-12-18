pub fn get_config_param_description(param_name: &str) -> String {
    match param_name {
        "font-family" | "font-family-bold" | "font-family-italic" | "font-family-bold-italic" => {
            "The font families to use.
                You can generate the list of valid values using the CLI:
                    path/to/ghostty/cli +list-fonts

            Changing this configuration at runtime will only affect new terminals,
            i.e. new windows, tabs, etc."
                .to_string()
        }
        "font-style" | "font-style-bold" | "font-style-italic" | "font-style-bold-italic" => {
            r#"The named font style to use for each of the requested terminal font
styles. This looks up the style based on the font style string advertised
by the font itself. For example, "Iosevka Heavy" has a style of "Heavy".

 You can also use these fields to completely disable a font style. If
 you set the value of the configuration below to literal "false" then
 that font style will be disabled. If the running program in the terminal
 requests a disabled font style, the regular font style will be used
 instead.

 These are only valid if its corresponding font-family is also specified.
 If no font-family is specified, then the font-style is ignored unless
 you're disabling the font style."#
                .to_string()
        }
        "font-feature" => r#"
 Apply a font feature. This can be repeated multiple times to enable
 multiple font features. You can NOT set multiple font features with
 a single value (yet).

 The font feature will apply to all fonts rendered by Ghostty. A
 future enhancement will allow targeting specific faces.

 A valid value is the name of a feature. Prefix the feature with a
 "-" to explicitly disable it. Example: "ss20" or "-ss20".

 To disable programming ligatures, use "-calt" since this is the typical
 feature name for programming ligatures. To look into what font features
 your font has and what they do, use a font inspection tool such as
 fontdrop.info.

 To generally disable most ligatures, use "-calt", "-liga", and "-dlig"
 (as separate repetitive entries in your config).
                "#
        .to_string(),
        "font-size" => "Font size in points".to_string(),
        "font-variation"
        | "font-variation-bold"
        | "font-variation-italic"
        | "font-variation-bold-italic" => r#"
                 A repeatable configuration to set one or more font variations values
 for a variable font. A variable font is a single font, usually
 with a filename ending in "-VF.ttf" or "-VF.otf" that contains
 one or more configurable axes for things such as weight, slant,
 etc. Not all fonts support variations; only fonts that explicitly
 state they are variable fonts will work.

 The format of this is "id=value" where "id" is the axis identifier.
 An axis identifier is always a 4 character string, such as "wght".
 To get the list of supported axes, look at your font documentation
 or use a font inspection tool.

 Invalid ids and values are usually ignored. For example, if a font
 only supports weights from 100 to 700, setting "wght=800" will
 do nothing (it will not be clamped to 700). You must consult your
 font's documentation to see what values are supported.

 Common axes are: "wght" (weight), "slnt" (slant), "ital" (italic),
 "opsz" (optical size), "wdth" (width), "GRAD" (gradient), etc.

                "#
        .to_string(),
        "font-codepoint-map" => r#"
 Force one or a range of Unicode codepoints to map to a specific named
 font. This is useful if you want to support special symbols or if you
 want to use specific glyphs that render better for your specific font.

 The syntax is "codepoint=fontname" where "codepoint" is either a
 single codepoint or a range. Codepoints must be specified as full
 Unicode hex values, such as "U+ABCD". Codepoints ranges are specified
 as "U+ABCD-U+DEFG". You can specify multiple ranges for the same font
 separated by commas, such as "U+ABCD-U+DEFG,U+1234-U+5678=fontname".
 The font name is the same value as you would use for "font-family".

 This configuration can be repeated multiple times to specify multiple
 codepoint mappings.

 Changing this configuration at runtime will only affect new terminals,
 i.e. new windows, tabs, etc."#
            .to_string(),
            "font-thicken" => "Draw fonts with a thicker stroke, if supported.  This is only supported currently on macOS.".to_string(),
            "adjust-cell-width" | "adjust-cell-height" | "adjust-font-baseline" | "adjust-underline-position" | "adjust-underline-thickness" | "adjust-strikethrough-position" | "adjust-strikethrough-thickness" => {
                r#"
                     All of the configurations behavior adjust various metrics determined
 by the font. The values can be integers (1, -1, etc.) or a percentage
 (20%, -15%, etc.). In each case, the values represent the amount to
 change the original value.

 For example, a value of "1" increases the value by 1; it does not set
 it to literally 1. A value of "20%" increases the value by 20%. And so
 on.

 There is little to no validation on these values so the wrong values
 (i.e. "-100%") can cause the terminal to be unusable. Use with caution
 and reason.

 Some values are clamped to minimum or maximum values. This can make it
 appear that certain values are ignored. For example, the underline
 position is clamped to the height of a cell. If you set the underline
 position so high that it extends beyond the bottom of the cell size,
 it will be clamped to the bottom of the cell.

 "adjust-cell-height" has some additional behaviors to describe:
 - The font will be centered vertically in the cell.
 - The cursor will remain the same size as the font.
 - Powerline glyphs will be adjusted along with the cell height so
   that things like status lines continue to look aligned."#.to_string()
            }
        "theme" => {
            r#"
                 A named theme to use. The available themes are currently hardcoded to
 the themes that ship with Ghostty. On macOS, this list is in the
 `Ghostty.app/Contents/Resources/themes` directory. On Linux, this
 list is in the `share/ghostty/themes` directory (wherever you installed
 the Ghostty "share" directory.

 To see a list of available themes, run `ghostty +list-themes`.

 Any additional colors specified via background, foreground, palette,
 etc. will override the colors specified in the theme.

 This configuration can be changed at runtime, but the new theme will
 only affect new cells. Existing colored cells will not be updated.
 Therefore, after changing the theme, you should restart any running
 programs to ensure they get the new colors.

 A future update will allow custom themes to be installed in
 certain directories.
                "#.to_string()
            }
        "background" => "Background color for the window".to_string(),
        "foreground" => "Foreground color for the window".to_string(),
        "selection-foreground" | "selection-background" => {
            "The foreground and background color for selection. If this is not
set, then the selection color is just the inverted window background
and foreground (note: not to be confused with the cell bg/fg).
"
            .to_string()
        }
        "selection-invert-fg-bg" => {
            r#"Swap the foreground and background colors of cells for selection.
 This option overrides the "selection-foreground" and "selection-background"
 options.

 If you select across cells with differing foregrounds and backgrounds,
 the selection color will vary across the selection.
"#.to_string()
            }
        "minimum-contrast" => {
            r#"The minimum contrast ratio between the foreground and background
 colors. The contrast ratio is a value between 1 and 21. A value of
 1 allows for no contrast (i.e. black on black). This value is
 the contrast ratio as defined by the WCAG 2.0 specification.

 If you want to avoid invisible text (same color as background),
 a value of 1.1 is a good value. If you want to avoid text that is
 difficult to read, a value of 3 or higher is a good value. The higher
 the value, the more likely that text will become black or white.

 This value does not apply to Emoji or images."#.to_string()
            }
        "palette" => {
            r#"Color palette for the 256 color form that many terminal applications
 use. The syntax of this configuration is "N=HEXCODE" where "n"
 is 0 to 255 (for the 256 colors) and HEXCODE is a typical RGB
 color code such as '#AABBCC'. The 0 to 255 correspond to the
 terminal color table.

 For definitions on all the codes:
 https://www.ditig.com/256-colors-cheat-sheet"#.to_string()
            }
        "cursor-color" => {
            "The color of the cursor.  If this is not set, a default will be chosen.".to_string()
            }
        "cursor-opacity" => {
            r#"The opacity level (opposite of transparency) of the cursor.
 A value of 1 is fully opaque and a value of 0 is fully transparent.
 A value less than 0 or greater than 1 will be clamped to the nearest
 valid value. Note that a sufficiently small value such as 0.3 may be
 effectively invisible and may make it difficult to find the cursor.
"#.to_string()
            }

        _ => "No description found".to_string(),
    }
}
