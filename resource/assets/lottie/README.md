# Lottie sample animations

The animations the Showcase's Lottie page offers in its picker, beside the hand-authored
`../hello.json`. Each is vendored **exactly as fetched** from Airbnb's Lottie repositories, which
are licensed under the [Apache License 2.0](https://www.apache.org/licenses/LICENSE-2.0). They
carry no copyright header of ours.

| File | Source | Upstream name |
| --- | --- | --- |
| `hamburger-arrow.json` | [airbnb/lottie-android](https://github.com/airbnb/lottie-android/tree/master/sample/src/main/assets) | `HamburgerArrow.json` |
| `lottie-logo.json` | airbnb/lottie-android, same directory | `Lottie Logo 1.json` |
| `heart.json` | [airbnb/lottie-ios](https://github.com/airbnb/lottie-ios/tree/master/Tests/Samples) | `Heart.json` |
| `watermelon.json` | airbnb/lottie-ios, same directory | `Watermelon.json` |
| `pin-jump.json` | airbnb/lottie-ios, same directory | `PinJump.json` |

`lottie("lottie/pin-jump")` loads one: the piece resolves `resource/assets/lottie/pin-jump.json`
on iOS and reads `assets/lottie/pin-jump.json` on Android. The facts the page shows beside each
file come from the piece's headless reader, and `dayscript/lottie-speed.yaml` asserts them.
