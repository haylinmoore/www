---
title: FIOS Home Router Emoji
description: changing the SSID of a Fios Home Router to an emoji
date: 2020-06-24
tags:
    - wifi
    - unicode
    - emoji
---

Setting the name/SSID of your WiFi to an emoji is fairly simple for the Fios Home Router. 
Simply log into the router, and go the page for setting the SSID. 

Once you are there open the developer console using `Ctrl+Shift+I` or by right clicking and clicking  the option labeled along the lines of `Inspect Element` or `Inspect`. 
After this simply paste the following code into the console. 
```javascript
validSSID = function(){return true}
```
This overrides the current validSSID function which is used to make sure that your SSID is made of the Fios required 1-32 alphanumerical ascii characters. 
This is not an actual technical requirements for SSIDs with the 2012 standard of 802.11 (Section 6.3.11.2.2) stating SSIDs simply have to be a max of 32 bytes.
Once this script is ran you should be able to type anything less than 32 bytes.
This tends to be at most 32 characters, but depending on the size of the Unicode characters used it can be much less.
For instance the fire emoji, `🔥`, is actually 4 bytes because of how UTF-8 works. 
This means that only 8 fire emojis could be used.
To get the size of a string simply use the site [https://mothereff.in/byte-counter](https://mothereff.in/byte-counter).

Have fun setting unusal SSIDs