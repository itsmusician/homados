# Changelog

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
