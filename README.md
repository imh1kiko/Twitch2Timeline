# Twitch2Timeline
A command-line tool written in rust to convert Twitch stream marker file (.csv) into Davinci Resolve marker file (.edl).

# How it works (and motivations)
Motivations were "I don't trust things to stay online forever, and prefer offline". The project that I'm referring to is [this one](https://enbyss.com/tools/marker-converter). It's more or less 1-to-1.

Technical jargon incoming:<br />Twitch records times as "`time`, `admin level` (which is redundant, as you need to be editor to add markers), `user` and `description`". However, edl files seem rather different. So we boil it down to barebones template. Then, using that template, we iterate over csv entries, and swap them out. Obviously there's some conversion happening (as edl files use `TC:HH:MM:SS`, instead of `HH:MM:SS`).<br />If you're curious, just check the `main.rs`. There's comments and probably lots of bad code.

# Usage
The usage is simple, and if you fail to enter it, it will prompt you with correct usage.

`Usage: Twitch2Timeline.exe <INPUT FILE> <MARKER COLOR> <HOUR OFFSET> <OUTPUT NAME>`<br />

`INPUT FILE` - This is your csv file. You can drag and drop this to your CLi.


`MARKER COLOR` - These are what color your marker will be. I didn't implement any fancy way of giving different colors, so all of them will be the same.
For now, the colors are what Davinci Resolve supports:
- Blue
- Cyan
- Green
- Red
- Yellow
- Pink
- Purple
- Fuchsia
- Rose
- Lavender
- Sky
- Mint
- Lemon
- Sand
- Cocoa
- Cream<br />
In case of misspelling or wrong usage, will default to blue.

`HOUR OFFSET` - Where your timeline starts at. The default is 0. So if you want 01:00:00:00, you'd use "1". Hopefully clear enough example.

`OUTPUT NAME` - Self-explanatory. A name for the file. No need for `.edl` suffix. Didn't code in the check for it either.

---

## FAQ
Q: Help! How do I get the CSV file (marker data) from Twitch?<br />
A: ![FAQ 1 Image](https://github.com/imh1kiko/Twitch2Timeline/raw/main/images/faq1.png)

Q: How do I use this in Davinci Resolve?<br />
A: Create a timeline, add your VOD, go to `Media` page, right click on the timeline you want to add the markers to, `Timelines → Import... → Timeline Markers from EDL`.

An image included to make it easier to understand.
![FAQ 2 Image](https://github.com/imh1kiko/Twitch2Timeline/raw/main/images/faq2.png)
Reason why you can't straight jump to adding markers is due to not letting.

