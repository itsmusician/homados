# Changelog

**0.0.3** - [2025.00.00] Description
</br>

    HI

    homados -- *.rs
    - All f32 values have been refactored to be f64 for increased precision, and to address an
        annoying bug related to this which affected many signal outputs negatively.

    homados -- main.rs
    - New "--GaindB" / "-G" argument allows the user to specify the gain scalar value in terms of
        dBFS as the unit. The short alias is Capital 'G', while the non-dBFS amplitude-based
        version uses a lowercase 'g' instead. The default value is 0dBFS.
    - New "--DurationSamples" / "-D" argument allows the user to specify the output duration in
        terms of samples as the unit. The short alias is Capital 'D', while the second-based
        version uses a lowercase 'd' instead. The default value is 480,000 samples. (10s in 48k)
    - New "--p2" argument allows a second generator-specific argument to be passed in
    - New "--p2dB" argument is the same as p2, but in dBFS units
    - The "--SoundDuration" argument has been renamed to "--DurationSeconds" for consistency
    - The "--Param1" argument has been renamed to "--p1" for consistency
    - The "--Param1db" argument has been renamed to "--p1dB" for consistency

    homados -- generator.rs
    - New sound types for the generator:
      - Pulse Width Sweep
        - Parameter 1 specifies the starting pulse width
        - Parameter 2 specifies the ending pulse width
    - Introduced groundwork for Dirac Comb Sweep modes
      - Currently disabled, exhibits poor results due to lack of anti-aliasing
        - Easily re-enabled through removing comments for the experimentally-curious!
    
    debug.sh
    - Added Partial Debugging Support
      - The original behavior is a full test
      - The user is now prompted whether to run a full test or not
        - If not, the sound and window type tests are individually prompted respectively, and then
            run according to the prompt responses.
      - Any combination of the partial tests may be run, including no test at all.
    - Added Pulse Wave to test cases (oopsie!)
    - Added new Pulse Width Sweep to test cases
    - There is now a test completion message
    
    homados Manual
    - Updated text to reflect updates as seen in the changelog

**0.0.2** - [2025.04.05] Saturday afternoon, starting to get quite warm out
</br>

    Hello!
    
    Lot's of big improvements this time around, a few which embarassingly should've been included
    on the initial release (Like forgetting to include all the fade window types in the manual...)

    homados now has a tagline:
    Signal should be simple.

    homados -- main.rs
    - All generator parameters are now internally passed in through a struct to simplify the 
        process as a whole and make it easier to add additional parameters in the future.
    - File Path and Name are now optional arguments, defaulting to a "homados Output" folder in
        the homados directory with the name "homados_output". They may be placed between any
        arguments, however the order must still first be the path, followed by the file name.
    - The "--Param1" argument now has a short alias of "-p"
    - Frequency arguments have been updated to base, minimum, and maximum parameters for various
        generator types as appropriate. The "-f" argument now refers to only the base frequency.
    - New "--Param1db" argument allows passing in dBFS values to be automatically converted to 
        amplitude values. Param1db takes precedence over Param1, so if both are passed the dBFS
        value will be used.
    - New "--Offset" / "-o" argument allows shifting select generator sound types around in time
        in units of seconds. Infrastructure has been created to allow a sample-based offset value
        in a future update. Currently, only the unit impulse sound type uses this value.
    
    homados -- generator.rs
    - All periodic sound types now use phase accumulator implementations
    - Optimized away some redundant operations in generator sound types
    - Re-enabled linear and logarithmic sine sweeps
    - New sound types for the generator:
      - Clipped Sine
        - Parameter 1 specifies the clipping threshold in amplitude.
        - Parameter 1 dB specifies the clipping threshold in dBFS.
      - Pulse Wave
        - Parameter 1 specifies the duty cycle of the pulse wave
      - Sharktooth Wave

    homados Manual
    - Updated text to reflect updates as seen in the changelog
    - Fixed missing sound types page
    - Version number included on each page at the bottom left
    - Reworded getting started and commands sections text
    
**0.0.1** - [2025.03.22] Almost 1 A.M.
</br>

    This is the very first version! Thanks for getting here early and roughing out such an early version.
    
    Every single line of code and included file is brand new / technically a change this update.

**0.0.0** - [Pre-History] Before the dawn of time (this repo)
</br>

    All the initial code and conceptualizing of this project in its infancy and prior to updates and revisions denoted above :)
