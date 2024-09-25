# Bouncy Feet

Try it: [app.bouncy-feet.ch](https://app.bouncy-feet.ch)

A web app that assists you in learning to shuffle dance. Bouncy Feet shows you
new moves and checks if you are doing it right.

The app can be installed as a Progressive Web App or just used directly like a
web page. It should work in all major browsers, on the phone or on laptops and
workstations. You only need a camera which you can position in a way where your
full body is clearly visible while you dance. Listings in common app stores are
in the working.

Check out the video below to see it in action.

https://github.com/user-attachments/assets/e327061d-69b1-40e4-9731-6cb322f77109

## Why would I need an app for dancing?

Let's be honest, dancing is best without interacting with your phone or laptop.
Just play some music and go. The app is designed with this principle in mind and
tries to support you without getting in your way.

**Easy Review**: After a dance, you can review your video recording. Bouncy Feet
enhances the recording with timestamps and body positions it detects on the
beat. If you like the video, save and share it. Otherwise, delete the video and
it is gone forever. (BouncyFeet is perfectly private and never uploads the
video, everything is done on your device only.)

**Browse a Collection of Moves**: An ever-growing list of shuffle moves are
available in the app to inspire you.

**Stay Motivated**: Incremental statistics counting how many steps you danced in
your lifetime using the app motivates you to stay fit and active.

**And more in the future**:
- Attach video tutorials to moves in the collection.
- Combine moves to choreographies. Learn them with an interactive audio guide.
  Share the choreography with friends to practice at home and next time you meet
  you will be able to dance it together in synchrony.
- Easy video editing with filters and custom 3D models dancing next to you or
  instead of you.

**But never forget**: Dancing is physical, not digital. An app will not replace
your visit at the dance studio where you meet your friends and sweat alongside
them. Bouncy Feet the app works best when you forget that it's even recording
you while you dance. Bouncy Feet is designed to be useful to you while
minimizing the screen time of you using it.

## About the Project

At its core, Bouncy Feet is an attempt of myself to create a product which
people actually want to use. Including myself, as the user number one.

In October 2023, I quit my fulltime job to "work on my own terms" without much
of a plan. After a few weeks, this idea of a dance app came to me and I decided
to start working on it for the majority of my time, while doing other gigs on the
side to keep my personal finances in balance.

In August 2024, Bouncy Feet GmbH was founded in Switzerland as a private limited
company, with the purpose to promote (shuffle) dance by means of software
assistance. Practically speaking, it's still just me running the show. But it
opens the door to commercial partnerships and brings legal clarity for an upcoming
release to the Play Store and the App Store.

It is still a bit unclear where this project will end up exactly. I am pretty
certain that to reach as many people as possible, the app should always be
available for free in some form. Monetization could work through content, like
courses by professional dance teachers, directly integrated with the software.
Paid premium features are also a possibility but let's see how things turn out.

Regarding tech, the project uses [SvelteKit](https://kit.svelte.dev/) for
everything UI, [MediaPipe](https://developers.google.com/mediapipe) with
pre-trained models for basic pose detection, and Rust (in the browser) for the
core business logic (dance detection).

## Progress Status

- 25 Sep 2024: Release version v0.5.2
    - The course lesson review page now shows which body parts are wrong for missed poses.
    - Add several prompt and in-between screen to make it easier to follow what is happening in the app.
    - Use a smaller model for the detection of the dance in the video.
      This should make tracking smoother, especially on less powerful devices, such as old phones.
      On the flip side, it will make detection less accurate in some circumstances.
    - Enabling experimental features now also shows systems statistics that can be useful to debug performance issues.
    - Various UI styling fixes and changes.
- 18 Sep 2024: Release version v0.5.1
    - Fix language selection based on locale configured on the client device.
    - Fix: Preserve icons in automated translation.
    - Server infrastructure changes. (Scoreboard is now hosted on stats.bouncy-feet.ch)
-  9 Sep 2024: Release version v0.5.0
    - Introduce courses, with a running man course as a first example.
      - Note: This is a poorly made example course for demonstration purposes. Actual courses will have better quality videos and better tracking.
      - A course has several lessons, each with an explanation video and a tracked exercise.
      - Hitting 60% on the traced exercise will grant the "absolved" mark on the course lesson.
      - The last lesson can be a step or choreography that's learned incrementally throughout the course.
    - Changed the style of the home screen quite a bit.
    - Many bug fixes, probably many new bugs.
- 30 Aug 2024: Overhaul README and change license
    - Describe the latest project goals, such as potential monetization ideas
      and take a clear stance on free and open-source software.
    - Relicense from permissive MIT / Apache 2.0 dual license to single a license, AGPL version 3.
- 28 Aug 2024: Released v0.4.1.
    - Improved dance animations: They now make a little jump between poses.
    - Change to live step detection:
        - Wait for proper positioning before detecting steps.
        - Voice countdown once the initial position has been reached.
        - Ignore frames where the dancer is not in the frame with legs and arms.
        - Count looking 90° in the wrong direction as an error, even if all angles match.
    - UI fixes:
      - Proper centering on wide screens.
- 30 July 2024: Released v0.4.0.
    - Live camera detection for a selected set of steps. (Running Man, Reverse Running Man, Kicking Reverse Running Man, Happy Feet.)
      - Learn mode: Learn each position without timing.
      - Train mode: Dance the move at your own speed with your own music.
    - Per step experience and levels that increase the more you dance it.
    - Steps are now categorized in groups: Melbourne Shuffle Basics, Cutting Shapes, Footwork, and Running Man variations.
    - A few more steps were added while some of the most basic steps are now hidden in the collection. (But still available in the choreography editor.)
    - A global leaderboard to compare how many steps you have recorded with other dancers. (Opt-in, by default zero data is transmitted.)
    - A profile settings page. Currently only allows to to opt-in to experimental features and manage the leaderboard participation settings.
    - Shared a quick promo video [on Reddit](https://www.reddit.com/r/shuffle/comments/1eglao3/bouncy_feet_app_update/) that shows the idea of dancing and leveling up each move in the app.
-  7 May 2024: Released v0.3.0 to the live demo.
    - Collection now shows *dances*, which are choreographies that consist of
      multiple steps in a row.
    - 4 demo dances for a start.
    - Create new dances and edit existing ones.
-  3 Apr 2024: Released v0.2.1 to the live demo.
    - More steps to the collection, now there are 13 steps.
    - Many improvements to drawing, such as z-perspective for limbs, the torso
      no longer being fixed to the center, and smooth shoulder/hip angles.
- 20 Mar 2024: Teaser Video on [Reddit](https://www.reddit.com/r/shuffle/comments/1bjdl1y/teaser_video_for_my_little_sideproject/)
- 19 Mar 2024: Created live demo with more stable features to https://app.bouncy-feet.ch.
- 26 Dec 2023: Started hosting a live demo on https://demos.jakobmeier.ch/bouncy-feet.
- 23 Dec 2023: Update README to better describe the project ambitions. The
  techstack with SvelteKit + Rust to create a PWA is mostly locked in now.
- 20 Nov 2023: Started using [SvelteKit](https://kit.svelte.dev/).
- 18 Nov 2023: Initialized repository under the name `bouncy-feet`.
- Oct - Nov 2023: Tried out the core concepts in a
  [prototype](https://github.com/jakmeier/dance-app-poc-playground).

# Free and Open-Source Commitment

Bouncy Feet is developed according to Free and Open-Source principles. I found
that my own opinions align well with those values, as I will elaborate a bit
more below.

I am a strong proponent of software with openly accessible source code, for two
main reasons.

1. I personally love it when I am able to look at the source code of software I
   use. It can be a great learning experience to look at real code and the
   process of how that code came to be.
2. I admire the community aspect of successful open-source projects. People use
   it to learn and practice their favorite technologies beyond what would be
   possible at a day-to-day job. This is the perfect counterweight to big
   cooperations in the software space.

I also believe that software should serve its users, rather than the oppress
them. It annoys me when software does things I don't want, without any way for
me to fix it. Consequently, I will avoid creating such software as much as I
can.

In other words, I support the free software movement and want this project to
adhere to its philosophy.

## License

This project is developed under a free and open-source license.
Specifically, AGPL version 3 or later. ([LICENSE](./AGPL-3.0-or-later.md))

In layman's terms, you may reuse the code of Bouncy Feet for other projects,
commercial or not, as long as you don't distribute that project in ways that
disrespect the freedom of their users. In particular, your project also has to
be free and open-source under AGPL version 3 or later, if you distribute it.

The purpose of this license is to ensure all users of the software developed for
the Bouncy Feet app will always have unrestricted access to the source code,
including the freedom to modify it to their desire. I hope that means any
attempt to produce derivative work with privacy invading data trackers or other
types of malware will simply be met by the community removing the offending code
and releasing the cleaned up version in a fork.

If you find the AGPL version 3 unsuitable for your use case, please contact
inbox@jakobmeier.ch and describe what you want to do with the software.
Additional licenses can be granted on a case-by-case basis. But for the general
public, it's only AGPL for now.

## Contribution

Contributions of all kinds are highly welcome!

If you want to help out with the dance content, or with translations, or
something else, please drop me a message inbox@jakobmeier.ch and I am more than
happy to discuss further. 

If you would like to help out on the coding side, that is great! Just to be
upfront about it, a lack of documentation might make it hard to get started.
Sadly, there is a chicken and egg problem here, since keeping good documentation
up to date is not worth my time until there is actual interest of contributors.

However, if you are willing to push through a lack of documentation, I am happy
to help you get started and create the necessary docs as we go. Even if you lack
experience using Rust and/or Svelte, I can help you learn. Ideally, introduce
yourself in an issue or email and let me know what you would like to work on and
we take it from there.

You may also have a look at
[issues with contribution encouraged](https://github.com/jakmeier/bouncy-feet/issues?q=is%3Aissue+is%3Aopen+label%3A%22contribution+encouraged%22).
These are typically long-term features which I know I want in the app eventually
but I don't have the capacity to work on them right now. If you want to work on
them, I will give my best to help you help me and make it a pleasant
collaboration experience for both of us.