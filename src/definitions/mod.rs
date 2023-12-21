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
        "cursor-style" => {
            r#"The style of the cursor. This sets the default style. A running
programn can still request an explicit cursor style using escape
sequences (such as CSI q). Shell configurations will often request
specific cursor styles.

Caveat: Shell integration currently defaults to always be a bar
In order to fix it, we probably would want to add something similar to Kitty's
shell integration options (no-cursor). For more information see:
https://sw.kovidgoyal.net/kitty/conf/#opt-kitty.shell_integration
"#.to_string()
            }
        "cursor-style-blink" => {
            r#"Sets the default blinking state of the cursor. This is just the
default state; running programs may override the cursor style
using DECSCUSR (CSI q).

If this is not set, the cursor blinks by default. Note that
this is not the same as a "true" value, as noted below.

If this is not set at all (null), then Ghostty will respect
DEC Mode 12 (AT&T cursor blink) as an alternate approach to
turning blinking on/off. If this is set to any value other
than null, DEC mode 12 will be ignored but DECSCUSR will still
be respected.
"#.to_string()
            }
        "cursor-text" => {
            r#"The color of the text under the cursor. If this is not set, a default
will be chosen.
"#.to_string()
            }
        "mouse-hide-while-typing" => {
            r#"Hide the mouse immediately when typing. The mouse becomes visible
again when the mouse is used. The mouse is only hidden if the mouse
cursor is over the active terminal surface.
"#.to_string()
            }
        "mouse-shift-capture" => {
            r#"Determines whether running programs can detect the shift key pressed
with a mouse click. Typically, the shift key is used to extend mouse
selection.

The default value of "false" means that the shift key is not sent
with the mouse protocol and will extend the selection. This value
can be conditionally overridden by the running program with the
XTSHIFTESCAPE sequence.

The value "true" means that the shift key is sent with the mouse
protocol but the running program can override this behavior with
XTSHIFTESCAPE.

The value "never" is the same as "false" but the running program
cannot override this behavior with XTSHIFTESCAPE. The value "always"
is the same as "true" but the running program cannot override this
behavior with XTSHIFTESCAPE.

If you always want shift to extend mouse selection even if the
program requests otherwise, set this to "never".
"#.to_string()
        }
        "background-opacity" => {
            r#" The opacity level (opposite of transparency) of the background.
 A value of 1 is fully opaque and a value of 0 is fully transparent.
 A value less than 0 or greater than 1 will be clamped to the nearest
 valid value.

 Changing this value at runtime (and reloading config) will only
 affect new windows, tabs, and splits.
"#.to_string()
        }
        "background-blur-radius" => {
            r#" A positive value enables blurring of the background when
 background-opacity is less than 1. The value is the blur radius to
 apply. A value of 20 is reasonable for a good looking blur.
 Higher values will cause strange rendering issues as well as
 performance issues.

 This is only supported on macOS.
:w
"#.to_string()
        }
        "unfocused-split-opacity" => {
            r#" The opacity level (opposite of transparency) of an unfocused split.
 Unfocused splits by default are slightly faded out to make it easier
 to see which split is focused. To disable this feature, set this
 value to 1.

 A value of 1 is fully opaque and a value of 0 is fully transparent.
 Because "0" is not useful (it makes the window look very weird), the
 minimum value is 0.15. This value still looks weird but you can at least
 see what's going on. A value outside of the range 0.15 to 1 will be
 clamped to the nearest valid value.
"#.to_string()
        }
        "unfocused-split-fill" => {
            r#"// The color to dim the unfocused split. Unfocused splits are dimmed by
// rendering a semi-transparent rectangle over the split. This sets
// the color of that rectangle and can be used to carefully control
// the dimming effect.
//
// This will default to the background color.
"# .to_string()
        }
        "command" => {
            r#" The command to run, usually a shell. If this is not an absolute path,
 it'll be looked up in the PATH. If this is not set, a default will
 be looked up from your system. The rules for the default lookup are:

   - SHELL environment variable
   - passwd entry (user information)

 This can contain additional arguments to run the command with.
 If additional arguments are provided, the command will be executed
 using "/bin/sh -c". Ghostty does not do any shell command parsing.

 If you're using the `ghostty` CLI there is also a shortcut
 to run a command with argumens directly: you can use the `-e`
 flag. For example: `ghostty -e fish --with --custom --args`.
"#.to_string()
        }
        "link" => {
            r#" Match a regular expression against the terminal text and associate
 clicking it with an action. This can be used to match URLs, file paths,
 etc. Actions can be opening using the system opener (i.e. "open" or
 "xdg-open") or executing any arbitrary binding action.

 Links that are configured earlier take precedence over links that
 are configured later.

 A default link that matches a URL and opens it in the system opener
 always exists. This can be disabled using "link-url".

 TODO: This can't currently be set!
"#.to_string()
        }
        "link-url" => {
            r#" Enable URL matching. URLs are matched on hover and open using the
 default system application for the linked URL.

 The URL matcher is always lowest priority of any configured links
 (see "link"). If you want to customize URL matching, use "link"
 and disable this.
"#.to_string()
        }
        "fullscreen" => {
            r#" Start new windows in fullscreen. This setting applies to new
 windows and does not apply to tabs, splits, etc. However, this
 setting will apply to all new windows, not just the first one.

 On macOS, this always creates the window in native fullscreen.
 Non-native fullscreen is not currently supported with this
 setting.
"#.to_string()
        }
        "title" => {
            r#" The title Ghostty will use for the window. This will force the title
 of the window to be this title at all times and Ghostty will ignore any
 set title escape sequences programs (such as Neovim) may send.
"#.to_string()
        }
        "class" => {
            r#" The setting that will change the application class value.

 This controls the class field of the WM_CLASS X11 property (when running
 under X11), and the Wayland application ID (when running under Wayland).

 Note that changing this value between invocations will create new, separate
 instances, of Ghostty when running with --gtk-single-instance=true. See
 that option for more details.

 The class name must follow the GTK requirements defined here:
 https://docs.gtk.org/gio/type_func.Application.id_is_valid.html

 The default is "com.mitchellh.ghostty".

 This only affects GTK builds.
"#.to_string()
        }
        "x11-instance-name" => {
            r#" This controls the instance name field of the WM_CLASS X11 property when
 running under X11. It has no effect otherwise.

 The default is "ghostty".

 This only affects GTK builds.
"#.to_string()
        }
        "working-directory" => {
            r#" The directory to change to after starting the command.

 This setting is secondary to the "window-inherit-working-directory"
 setting. If a previous Ghostty terminal exists in the same process,
 "window-inherit-working-directory" will take precedence. Otherwise,
 this setting will be used. Typically, this setting is used only
 for the first window.

 The default is "inherit" except in special scenarios listed next.
 On macOS, if Ghostty can detect it is launched from launchd
 (double-clicked) or `open`, then it defaults to "home".
 On Linux with GTK, if Ghostty can detect it was launched from
 a desktop launcher, then it defaults to "home".

 The value of this must be an absolute value or one of the special
 values below:

   - "home" - The home directory of the executing user.
   - "inherit" - The working directory of the launching process.

"#.to_string()
        }
        "keybind" => {
            r#" Key bindings. The format is "trigger=action". Duplicate triggers
 will overwrite previously set values.

 Trigger: "+"-separated list of keys and modifiers. Example:
 "ctrl+a", "ctrl+shift+b", "up". Some notes:

   - modifiers cannot repeat, "ctrl+ctrl+a" is invalid.
   - modifiers and keys can be in any order, "shift+a+ctrl" is weird,
     but valid.
   - only a single key input is allowed, "ctrl+a+b" is invalid.

 Valid modifiers are "shift", "ctrl" (alias: "control"),
 "alt" (alias: "opt", "option"), and "super" (alias: "cmd", "command").
 You may use the modifier or the alias. When debugging keybinds,
 the non-aliased modifier will always be used in output.

 Action is the action to take when the trigger is satisfied. It takes
 the format "action" or "action:param". The latter form is only valid
 if the action requires a parameter.

   - "ignore" - Do nothing, ignore the key input. This can be used to
     black hole certain inputs to have no effect.
   - "unbind" - Remove the binding. This makes it so the previous action
     is removed, and the key will be sent through to the child command
     if it is printable.
   - "csi:text" - Send a CSI sequence. i.e. "csi:A" sends "cursor up".
   - "esc:text" - Send an Escape sequence. i.e. "esc:d" deletes to the
     end of the word to the right.

 Some notes for the action:

   - The parameter is taken as-is after the ":". Double quotes or
     other mechanisms are included and NOT parsed. If you want to
     send a string value that includes spaces, wrap the entire
     trigger/action in double quotes. Example: --keybind="up=csi:A B"

 There are some additional special values that can be specified for
 keybind:

   - `keybind = clear` will clear all set keybindings. Warning: this
     removes ALL keybindings up to this point, including the default
     keybindings.

"#.to_string()
        }
        "window-padding-x" => {
            r#" Window padding. This applies padding between the terminal cells and
 the window border. The "x" option applies to the left and right
 padding and the "y" option is top and bottom. The value is in points,
 meaning that it will be scaled appropriately for screen DPI.

 If this value is set too large, the screen will render nothing, because
 the grid will be completely squished by the padding. It is up to you
 as the user to pick a reasonable value. If you pick an unreasonable
 value, a warning will appear in the logs.
"#.to_string()
        }
        "window-padding-y" => {
            r#" Window padding. This applies padding between the terminal cells and
 the window border. The "x" option applies to the left and right
 padding and the "y" option is top and bottom. The value is in points,
 meaning that it will be scaled appropriately for screen DPI.

 If this value is set too large, the screen will render nothing, because
 the grid will be completely squished by the padding. It is up to you
 as the user to pick a reasonable value. If you pick an unreasonable
 value, a warning will appear in the logs.
"#.to_string()
        }
        "window-padding-balance" => {
            r#" The viewport dimensions are usually not perfectly divisible by
 the cell size. In this case, some extra padding on the end of a
 column and the bottom of the final row may exist. If this is true,
 then this extra padding is automatically balanced between all four
 edges to minimize imbalance on one side. If this is false, the top
 left grid cell will always hug the edge with zero padding other than
 what may be specified with the other "window-padding" options.

 If other "window-padding" fields are set and this is true, this will
 still apply. The other padding is applied first and may affect how
 many grid cells actually exist, and this is applied last in order
 to balance the padding given a certain viewport size and grid cell size.
"#.to_string()
        }
        "window-inherit-working-directory" => {
            r#" If true, new windows and tabs will inherit the working directory of
 the previously focused window. If no window was previously focused,
 the default working directory will be used (the "working-directory"
 option)."#.to_string()
        }
        "window-inherit-font-size" => {
            r#" If true, new windows and tabs will inherit the font size of the previously
 focused window. If no window was previously focused, the default
 font size will be used. If this is false, the default font size
 specified in the configuration "font-size" will be used.
"#.to_string()
        }
        "window-decoration" => {
            r#" If false, windows won't have native decorations, i.e. titlebar and
 borders.
"#.to_string()
        }
        "window-theme" => {
            r#" The theme to use for the windows. The default is "system" which
 means that whatever the system theme is will be used. This can
 also be set to "light" or "dark" to force a specific theme regardless
 of the system settings.

 This is currently only supported on macOS and linux.
"#.to_string()
        }
        "window-height" => {
            r#" The initial window size. This size is in terminal grid cells by default.

 We don't currently support specifying a size in pixels but a future
 change can enable that. If this isn't specified, the app runtime will
 determine some default size.

 Note that the window manager may put limits on the size or override
 the size. For example, a tiling window manager may force the window
 to be a certain size to fit within the grid. There is nothing Ghostty
 will do about this, but it will make an effort.

 This will not affect new tabs, splits, or other nested terminal
 elements. This only affects the initial window size of any new window.
 Changing this value will not affect the size of the window after
 it has been created. This is only used for the initial size.

 BUG: On Linux with GTK, the calculated window size will not properly
 take into account window decorations. As a result, the grid dimensions
 will not exactly match this configuration. If window decorations are
 disabled (see window-decorations), then this will work as expected.

 Windows smaller than 10 wide by 4 high are not allowed.
"#.to_string() 
        }
        "window-width" => {
            r#" The initial window size. This size is in terminal grid cells by default.

 We don't currently support specifying a size in pixels but a future
 change can enable that. If this isn't specified, the app runtime will
 determine some default size.

 Note that the window manager may put limits on the size or override
 the size. For example, a tiling window manager may force the window
 to be a certain size to fit within the grid. There is nothing Ghostty
 will do about this, but it will make an effort.

 This will not affect new tabs, splits, or other nested terminal
 elements. This only affects the initial window size of any new window.
 Changing this value will not affect the size of the window after
 it has been created. This is only used for the initial size.

 BUG: On Linux with GTK, the calculated window size will not properly
 take into account window decorations. As a result, the grid dimensions
 will not exactly match this configuration. If window decorations are
 disabled (see window-decorations), then this will work as expected.

 Windows smaller than 10 wide by 4 high are not allowed.
"#.to_string()
        }
        "window-step-resize" => {
            r#" Resize the window in discrete increments of the focused surface's
 cell size. If this is disabled, surfaces are resized in pixel increments.
 Currently only supported on macOS.
"#.to_string()
        }
        "gtk-titlebar" => {
            r#" When enabled, the full GTK titlebar is displayed instead of your window
 manager's simple titlebar. The behavior of this option will vary with your
 window manager.

 This option does nothing when window-decoration is false or when running
 under MacOS.

 Changing this value at runtime and reloading the configuration will only
 affect new windows.
"#.to_string()
        }
        "clipboard-read" | "clipboard-write" => {
            r#" Whether to allow programs running in the terminal to read/write to
 the system clipboard (OSC 52, for googling). The default is to
 allow clipboard reading after prompting the user and allow writing
 unconditionally.
"#.to_string()
        }
        "clipboard-trim-trailing-spaces" => {
            r#" Trims trailing whitespace on data that is copied to the clipboard.
 This does not affect data sent to the clipboard via "clipboard-write".
"#.to_string()
        }
        "clipboard-paste-protection" => {
            r#" Require confirmation before pasting text that appears unsafe. This helps
 prevent a "copy/paste attack" where a user may accidentally execute unsafe
 commands by pasting text with newlines.
"#.to_string()
        }
        "clipboard-paste-bracketed-safe" => {
            r#" If true, bracketed pastes will be considered safe. By default,
 bracketed pastes are considered safe. "Bracketed" pastes are pastes
 while the running program has bracketed paste mode enabled (a setting
 set by the running program, not the terminal emulator).
"#.to_string()
        }
        "image-storage-limit" => {
            r#" The total amount of bytes that can be used for image data (i.e.
 the Kitty image protocol) per terminal scren. The maximum value
 is 4,294,967,295 (4GB). The default is 320MB. If this is set to zero,
 then all image protocols will be disabled.

 This value is separate for primary and alternate screens so the
 effective limit per surface is double.
"#.to_string()
        }
        "copy-on-select" => {
            r#" Whether to automatically copy selected text to the clipboard. "true"
 will only copy on systems that support a selection clipboard.

 The value "clipboard" will copy to the system clipboard, making this
 work on macOS. Note that middle-click will also paste from the system
 clipboard in this case.

 Note that if this is disabled, middle-click paste will also be
 disabled.
"#.to_string()
        }
        "click-repeat-interval" => {
            r#" The time in milliseconds between clicks to consider a click a repeat
 (double, triple, etc.) or an entirely new single click. A value of
 zero will use a platform-specific default. The default on macOS
 is determined by the OS settings. On every other platform it is 500ms.
"#.to_string()
        }
        "config-file" => {
            r#" Additional configuration files to read. This configuration can be repeated
 to read multiple configuration files. Configuration files themselves can
 load more configuration files. Paths are relative to the file containing
 the `config-file` directive. For command-line arguments, paths are
 relative to the current working directory.

 Cycles are not allowed. If a cycle is detected, an error will be logged
 and the configuration file will be ignored.
"#.to_string()
        }
        "confirm-close-surface" => {
            r#" Confirms that a surface should be closed before closing it. This defaults
 to true. If set to false, surfaces will close without any confirmation.
"#.to_string()
        }
        "quit-after-last-window-closed" => {
            r#" Whether or not to quit after the last window is closed. This defaults
 to false. Currently only supported on macOS. On Linux, the process always
 exits after the last window is closed.
"#.to_string()
        }
        "shell-integration" => {
            r#" Whether to enable shell integration auto-injection or not. Shell
 integration greatly enhances the terminal experience by enabling
 a number of features:

   * Working directory reporting so new tabs, splits inherit the
     previous terminal's working directory.
   * Prompt marking that enables the "jump_to_prompt" keybinding.
   * If you're sitting at a prompt, closing a terminal will not ask
     for confirmation.
   * Resizing the window with a complex prompt usually paints much
     better.

 Allowable values are:

   * "none" - Do not do any automatic injection. You can still manually
     configure your shell to enable the integration.
   * "detect" - Detect the shell based on the filename.
   * "fish", "zsh" - Use this specific shell injection scheme.

 The default value is "detect".
"#.to_string()
        }
        "shell-integration-features" => {
            r#" Shell integration features to enable if shell integration itself is enabled.
 The format of this is a list of features to enable separated by commas.
 If you prefix a feature with "no-" then it is disabled. If you omit
 a feature, its default value is used, so you must explicitly disable
 features you don't want.

 Available features:

   - "cursor" - Set the cursor to a blinking bar at the prompt.

 Example: "cursor", "no-cursor""#.to_string()
        }
        "osc-color-report-format" => {
            r#" Sets the reporting format for OSC sequences that request color information.
 Ghostty currently supports OSC 10 (foreground), OSC 11 (background), and OSC
 4 (256 color palette) queries, and by default the reported values are
 scaled-up RGB values, where each component are 16 bits. This is how most
 terminals report these values. However, some legacy applications may require
 8-bit, unscaled, components. We also support turning off reporting
 alltogether. The components are lowercase hex values.

 Allowable values are:

   * "none" - OSC 4/10/11 queries receive no reply
   * "8-bit" - Color components are return unscaled, i.e. rr/gg/bb
   * "16-bit" - Color components are returned scaled, e.g. rrrr/gggg/bbbb

 The default value is "16-bit".
"#.to_string()
        }
        "vt-kam-allowed" => {
            r#" If true, allows the "KAM" mode (ANSI mode 2) to be used within
 the terminal. KAM disables keyboard input at the request of the
 application. This is not a common feature and is not recommended
 to be enabled. This will not be documented further because
 if you know you need KAM, you know. If you don't know if you
 need KAM, you don't need it.
"#.to_string()
        }
        "customer-shader" => {
            r#" Custom shaders to run after the default shaders. This is a file path
 to a GLSL-syntax shader for all platforms.

 WARNING: Invalid shaders can cause Ghostty to become unusable such as by
 causing the window to be completely black. If this happens, you can
 unset this configuration to disable the shader.

 On Linux, this requires OpenGL 4.2. Ghostty typically only requires
 OpenGL 3.3, but custom shaders push that requirement up to 4.2.

 The shader API is identical to the Shadertoy API: you specify a `mainImage`
 function and the available uniforms match Shadertoy. The iChannel0 uniform
 is a texture containing the rendered terminal screen.

 If the shader fails to compile, the shader will be ignored. Any errors
 related to shader compilation will not show up as configuration errors
 and only show up in the log, since shader compilation happens after
 configuration loading on the dedicated render thread.  For interactive
 development, use Shadertoy.com.

 This can be repeated multiple times to load multiple shaders. The shaders
 will be run in the order they are specified.

 Changing this value at runtime and reloading the configuration will only
 affect new windows, tabs, and splits.
"#.to_string()
        }
        "customer-shader-animation" => {
            r#" If true (default), the focused terminal surface will run an animation
 loop when custom shaders are used. This uses slightly more CPU (generally
 less than 10%) but allows the shader to animate. This only runs if there
 are custom shaders.

 If this is set to false, the terminal and custom shader will only render
 when the terminal is updated. This is more efficient but the shader will
 not animate.

 This value can be changed at runtime and will affect all currently
 open terminals.
"#.to_string()
        }
        "macos-non-native-fullscren" => {
            r#" If anything other than false, fullscreen mode on macOS will not use the
 native fullscreen, but make the window fullscreen without animations and
 using a new space. It's faster than the native fullscreen mode since it
 doesn't use animations.

 Allowable values are:

   * "visible-menu" - Use non-native macOS fullscreen, keep the menu bar visible
   * "true" - Use non-native macOS fullscreen, hide the menu bar
   * "false" - Use native macOS fullscreeen
"#.to_string()
        }
        "macos-option-as-alt" => {
            r#" If true, the Option key will be treated as Alt. This makes terminal
 sequences expecting Alt to work properly, but will break Unicode
 input sequences on macOS if you use them via the alt key. You may
 set this to false to restore the macOS alt-key unicode sequences
 but this will break terminal sequences expecting Alt to work.

 Note that if an Option-sequence doesn't produce a printable
 character, it will be treated as Alt regardless of this setting.
 (i.e. alt+ctrl+a).

 This does not work with GLFW builds.
"#.to_string()
        }
        "gtk-single-instance" => {
            r#" If true, the Ghostty GTK application will run in single-instance mode:
 each new `ghostty` process launched will result in a new window if there
 is already a running process.

 If false, each new ghostty process will launch a separate application.

 The default value is "desktop" which will default to "true" if Ghostty
 detects it was launched from the .desktop file such as an app launcher.
 If Ghostty is launched from the command line, it will default to "false".

 Note that debug builds of Ghostty have a separate single-instance ID
 so you can test single instance without conflicting with release builds.
"#.to_string()
        }
        "gtk-wide-tabs" => {
            r#" If true (default), then the Ghostty GTK tabs will be "wide." Wide tabs
 are the new typical Gnome style where tabs fill their available space.
 If you set this to false then tabs will only take up space they need,
 which is the old style.
"#.to_string()
        }
        "gtk-adwaita" => {
            r#" If true (default), Ghostty will enable libadwaita theme support. This
 will make `window-theme` work properly and will also allow Ghostty to
 properly respond to system theme changes, light/dark mode changing, etc.
 This requires a GTK4 desktop with a GTK4 theme.

 If you are running GTK3 or have a GTK3 theme, you may have to set this
 to false to get your theme picked up properly. Having this set to true
 with GTK3 should not cause any problems, but it may not work exactly as
 expected.

 This configuration only has an effect if Ghostty was built with
 libadwaita support.
"#.to_string()
        }
        "desktop-notifications" => {
            r#" If true (default), applications running in the terminal can show desktop
 notifications using certain escape sequences such as OSC 9 or OSC 777.
"#.to_string()
        }

        _ => "No description found".to_string(),
    }
}
