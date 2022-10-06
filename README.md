# brightness_control
Code to manipulate brightness for monitors

Sets the same brightness on all connected monitors

requires xrandr

How to use: 

compile release and place the binary file in /usr/bin/

example of use:

Increase by 10%:
```
brightness_control 0.1
```

Decrease by 0.1:
```
brightness_control -0.1
```