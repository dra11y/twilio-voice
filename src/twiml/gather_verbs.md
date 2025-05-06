#### Nest other verbs

You can nest the following verbs within `<Gather>`:
- [`<Pause>`](https://www.twilio.com/docs/voice/twiml/pause)
- [`<Play>`](https://www.twilio.com/docs/voice/twiml/play)
- [`<Say>`](https://www.twilio.com/docs/voice/twiml/say)

When a `<Gather>` contains nested `<Say>` or `<Play>` verbs, the `timeout` begins either after the audio completes or when the caller presses their first key. If `<Gather>` contains multiple `<Play>` verbs, Twilio retrieves the contents of all files before the `<Play>` begins.

Documentation: https://www.twilio.com/docs/voice/twiml/gather#nest-other-verbs
