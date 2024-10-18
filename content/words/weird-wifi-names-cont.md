---
title: Invalid UTF8 characters as SSIDs
description: comparing how different devices handle invalid UTF8 characters
date: 2020-07-03
tags:
    - wifi
    - unicode
    - emoji
---

This is a continuation of my previous post [Comparing how different devices display the SSID "á̶̛̛̓̿̈͐͆̐̇̒̑̈́͘͝aaa"](/posts/weird-wifi-name-display/). After posting it on HackerNews I got lots of feedback. The key one was something that I had sadly missed when I originally started this project. When the WiFi name of "á̶̛̛̓̿̈͐͆̐̇̒̑̈́͘͝aaa" got shortened down to a one byte before the second "a" due to the 32 byte/octet limit of SSIDs. The issue was this character is actually two bytes wide, so the first part of the character stayed, with the second byte of it missing. The character in question was ◌̈́, with the hex codepoint cd84. Looking at the raw hex of the SSID you can see it ends in cd, `61ccb6cc81cc93ccbfcc88cc9bcc9bcd90cd98cd86cc90cd9dcc87cc92cc91cd`. For those curious about how that would be rendered on your device here it is "á̶̛̛̓̿̈͐͆̐̇̒̑͘͝�". After realizing this I decided my previous comparisons of how devices reacted to my weird unicode SSID was unfair. The SSID this time is "á̶̛̛̓̿̈͐͆̐̇̒̑͘͝a" which is almost the exact same but that last two byte long character was replaced with an "a". This was to keep the SSID at 32 octets long.
Looking at the new hex of the SSID `61ccb6cc81cc93ccbfcc88cc9bcc9bcd90cd98cd86cc90cd9dcc87cc92cc9161` you can see that is is identical baring the `61` instead of a `cd` at the end. 

Below are the previous photos of the previous SSID "á̶̛̛̓̿̈͐͆̐̇̒̑͘͝�" with the new SSID "á̶̛̛̓̿̈͐͆̐̇̒̑͘͝a" under it. They are also grouped the same as previously to keep continuity.

Galaxy S8 running Android 9 with Kernel 4.4.153
![](/assets/img/iosWifiBug/android.jpg)
![](/assets/img/iosWifiBug/d2/android.jpg)

Amazon Firestick
![](/assets/img/iosWifiBug/firestick.jpg)
![](/assets/img/iosWifiBug/d2/firestick.jpg)

As before the Galaxy S8 and the Firestick both handled it perfectly with there being no issues with how it rendered besides vertical cutoff which makes sense.

iPhone running iOS 13.5.1
![](/assets/img/iosWifiBug/iphone-ios1351.jpg)
![](/assets/img/iosWifiBug/d2/iphone-ios1351.jpg)

Apple TV Second Generation
![](/assets/img/iosWifiBug/appletvgen2.jpg)
![](/assets/img/iosWifiBug/d2/appletvgen2.jpg)

These results were interesting. It showed that if the UTF-8 character in an SSID is invalid then IOS falls back to the [Mac OS Roman](https://en.wikipedia.org/wiki/Mac_OS_Roman) character set, but since the new SSID is valid UTF-8 iOS properly renderers the SSID.

2012 Macbook running High Sierra 10.13.6
![](/assets/img/iosWifiBug/d2/macos.jpg)

Unlike last time the Macbook actually showed the network. I believe last time instead of falling back to Mac OS Roman like iOS the Macbook just treated the SSID as spam or noise and dropped it

Windows 10 Pro 10.0.19041
![](/assets/img/iosWifiBug/windows10.png)
![](/assets/img/iosWifiBug/d2/windows10.jpg)

Windows was able to properly handle this SSID as opposed to falling back to [Windows-1252](https://en.wikipedia.org/wiki/Windows-1252) like it did previously.

Chromebook running ChromeOS 83.0.4103.97
![](/assets/img/iosWifiBug/chromeos.jpg)
![](/assets/img/iosWifiBug/d2/chromeos.jpg)

Unlike last time the Chromebook was able to render this perfectly with no question marks.

Kindle Paperwhite running Firmware 5.10.2
![](/assets/img/iosWifiBug/kindlepaperwhite.jpg)
![](/assets/img/iosWifiBug/d2/kindlepaperwhite.jpg?)

Vizio M55-C2 TV
![](/assets/img/iosWifiBug/viziom55-c2.jpg)
![](/assets/img/iosWifiBug/d2/viziom55-c2.jpg)

Unlike last time the kindle was able to show the SSID without escape any of the characters. 
Sadly the Vizio was not able to show the SSID and just fell back to escaped hex.

The results from changing the SSID showed a couple things, namely devices really do not like it when they are given invalid or incomplete UTF-8 codes and lots of devices handle UTF-8 text much better than I thought they would.
It also showed that some devices fallback to their native 8 bit encoding formats when they see an invalid UTF-8 codepoint, there is probably some room for exploitation here like inserting escape keys.
I originally was not expecting the kindle to work at all, nor the chromebook after seeing the slew of question marks originally. Android was also found to be at handling text with invalid UTF-8 characters it just removed them as opposed to switching up formatting, falling back to hex, or erroring out with all question marks. 

Thank you to everyone who provided valuable feedback on the previous post. I have more research I plan ond doing with this in the future.

Lastly if anyone would like to run this test themselves and send me the comparison results to be added my email is `me (at) hamptonmoore (This key > not shifted) com`.