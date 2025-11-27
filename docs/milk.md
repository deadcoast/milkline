# `milk` the desktop audio visual media buddy

## DEVELOPER AND PROJECT FOUNDATIONS FOR `milk`

`milk` is designed for an age past, when home computing tech was jamming a floppy disk into the drive, and  hoping theres only one disk to the set.

It was a warm age, a simple age, the computing defined in the 2000's was one of simplicity and comfort. I can even go as far as to say the majority of home computer users lived in a state of blissful ignorance. Did I know that downloading 50cent-in-da-club(2) with no extension defined was risky? Yes, did I care? No;

*I wanted the damn song*

The laws of the land inside the world-wide-web were still being written, and we as users just didnt know what the consequences were, and to be fair back then the consequences were nothing like today:

```
1. Oh no, grandma clicked another million doller lottery pop up ->
2. Now Grandma has a virus that opens more adds ->
3. Grandma is now hacked, oh well ->
```

> That was the majority of the cases before personal life got so tangled up in the internet. Not the best, but no one was stealing identities, or ransomware and holding you hostage.

Modern day is the information age, and ironically enough, we have all the information in the world, but we choose to use none of it:

  *ignorance is bliss*

## Why `milk`

We all remember the days of getting home, hoping no one uses the phone so your dial up stays on after it chugs to life. Those warm cozy days on icq talking with friends, getting your homework done talking to a paperclip that woulnd go away in counter strike 1.3, ventrilo, and when browsing the internet was unpredictable, anyone have lemons for the party?

## Inspiration and Growth - The meaning behind it all

> `pardodyCode` representation of what inspired me to create `milk`

```parody
#milk.yaml
milk:"required":[
    winamp:"inspired",
        cow:"aquired",
        farmer:"hired"
    ]
```

Thats the era, so heres the tech: `milk` is designed to be a 2000's style front end, with a simple `Clippit` style buddy. The design for this buddy will be simply a box with a 50's style animated eyes and mouth. very simple. no ai, just code. The buddy is designed to run simple tasks like:

- prompting for an input path for the media library file
- prompting for youtube / spotify credentials and(ONLY IF REQUIRED)api key

im a really big fan of the era of winamp style computing applications. my two favourite designs were winamp(my main inspiration as a developer) and icq(beautiful ui design, sound design`for the eras limitations`).

I want to create something similar, but a wrapper application. it will be purely for stylistic purposes but its own application with a front and back end none the less.

[1](design_request) - Application Design

Create a winamp inspired application that acts as a front end for either youtube or spotify or both. I am not sure the limitations of what the spotify or youtube api allow you, such as:
- How much access with the API's provide us? Enough for what we need?
- Will the API allow us to utilize the data for our front end if the user logs in with credentials?
- The entire purpose of `milk` breaks down into two major purposes:

[1.a](quality_of_life) - `quality_of_life`
- Comprehensive development and planning into User quality of life, not for bloat, or redundant features, but for features that directly reflect and effect the core ones we have laid out.

[1.b](visual_aesthetics) - `visual_aesthetics`
- Visual Customization and Aesthetics, for the sake of the user, and the user experience. We are trying to recreate winamp as per, or re use their skin assets.

[1.c](user_experience) - `user_experience`
- The user experience is the most important part of the design, and the design is the most important part of the user experience. We are trying to recreate not only the look of winamp, but the feel it had, the feelings the world had back then.

[2] `milk` should not introduce additional complexity for the user, in almost all cases that is just bad development.
- The system is designed to provide additional complexity, in its own package, streamlined for user quality of life, and audio aesthetics.

[2.a](finding_design_balance) - `user_experice != quality_of_life`

- It takes more development to create a higher quality of life. Usualy if it makes it easier for the user, its more comprehensive work for us. That is whyt we must be comprehensive from the start.

(2.b)
    `if user_experience == quality_of_life`
- The system, and the user should be in harmony. If any part falls out of place(the design and aesthetics, quality of life, user experience, back end) then the ecosystem will fail.
- If the design is `**aboove and beyond**` as per spec everything else falls into place:

> [!NOTE]
> Desiging and completing the functions catered to the [three aspects]((a),(b),(c)) are essentiial to success.
> The applications simplicity, requires the simplicity to be rigerruously, comprehensivley, and creatively developed to its maximum extent, ot else `milk` will fail as an application and a project as a whole.

ENTERTAINING BALANCE IS IMPERATIVE:
- The quality of life, visual aesthetics, and user experience will only manifest through complete development scope and harmony.
Both aspects are very important, and neither can be ignored.
    - The design simplicity requires us to over develop the experience so they have the smallest margin for error possible.
    - This will allow us(require us) to modulate the experience for future dev plugins.so that what they listen to music on youtube(or spotify), the data, song titles, metadata, audio visualizer will synchronize on the `milk` UI.
