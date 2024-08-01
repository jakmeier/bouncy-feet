# Bouncy Feet

A web app that assists you in learning to shuffle dance. Bouncy Feet shows you
new moves and checks if you are doing it right.

The app can be installed as a Progressive Web App or just used directly like a
web page. It should work in all major browsers, on the phone or on laptops and
workstations. You only need a camera which you can position in a way where your
full body is clearly visible while you dance.

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

It is unclear where this project will end up. But right now, I am convinced the
first version of the app will be free for everyone and without adds. It might
make sense to add monetization later. But only if this ever turns from a fun
project to something that needs to be profitable to put food on the table for me
and others. But that is a far dream and too unlikely at this moment to be worth
thinking about too much.

Regarding tech, the project uses [SvelteKit](https://kit.svelte.dev/) for
everything UI, [MediaPipe](https://developers.google.com/mediapipe) with
pre-trained models for basic pose detection, and Rust (in the browser) for the
core business logic (dance detection).

## Progress Status

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

# Open-Source Commitment

I am a strong proponent of open-source software for two main reasons.

1. I personally love it when I am able to look at the source code of software I
   use. It can be a great learning experience to look at real code and the
   process of how that code came to be.
2. I admire the community aspect of successful open-source projects. People use
   it to learn and practice their favorite technologies beyond what would be
   possible at a day-to-day job. This is the perfect counterweight to big
   cooperations in the software space.

With this mindset, I want this project to be as open and welcoming as possible.
Even in the unlikely event where I want to monetize some form of Bouncy Feet, I
would still want to keep all the source code free and open-source. I would want
to monetize the content inside the app (think dance courses), if anything. But
access to the software itself should always remain free.

## License

This project is developed under permissive licenses. You may freely reuse the
code for your own projects. If you do, I would love to hear about it. Open an
issue, say hello and describe your project. Or drop me a private message. Of
course, there is no obligation to do so but I would appreciate it.

For exact licensing details, please refer to the license files. There are
multiple licenses available to choose from, just to give you more options and
have this code fit better in your project. If you find the licenses available
unsuitable for your project, please open an issue.

## Contribution

The current status is **source available** without proper documentation that
would be necessary to encourage a wider community to contribute. Hopefully, this
will improve over time. But if the lack of documentation does not bother you, I
am happy to answer pull requests and issues if you open them.

If you are interested in helping out by implementing features, the first place
to look is the list of [issues with contribution
encouraged](https://github.com/jakmeier/bouncy-feet/issues?q=is%3Aissue+is%3Aopen+label%3A%22contribution+encouraged%22).
These are typically long-term features which I know I want in the app eventually
but I don't have the capacity to work on them right now. If you want to work on
them, I will give my best to help you help me and make it a pleasant
collaboration experience for both of us.

But the list of issues is never complete and there are certainly other ways to
contribute, too. If you are interested in getting involved but don't see an
issue that suits you, please drop me a quick message at inbox@jakobmeier.ch and
describe your level of experience with Rust/Svelte and let me know what kind of
contribution interests you.

I myself have decent experience using Rust and can also mentor you a bit on your
journey learning it if that's what you are looking for. But with Svelte, I am
very new and could probably use some mentoring by you. :P Or we can learn it
side by side, you don't need to be professional with Svelte to contribute.

Oh and there are certainly non-technical ways to contribute, too! If you want to
help out with the dance content, or with translations, or something else, please
drop me a message inbox@jakobmeier.ch and I am more than happy to discuss
further.


