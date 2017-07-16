stm32f3-liar-test
=================

This is some work-in-progress test code for the `liar benchmarking crate
<https://github.com/ranweiler/liar>`_. At present, it is able to compile and
run liar on an STM32F3 Discovery board, benchmark short functions using the
Cortex-M DWT cycle counter, and present very simple results via semihosting.

This is built on top of Jorge Aparicio's embedded Rust work, including the
`xargo <https://github.com/japaric/xargo>`_ cross-compilation tool and his
`Cortex-M <https://github.com/japaric/cortex-m-rt>`_ runtime. If you can get his
`quickstart examples <http://blog.japaric.io/quickstart/>`_ running, you should
be able to run this in much the same way.

Output
------
Right now, the code just dumps the liar::no_std::runner::Samples struct for each
test run. I modified my copy of liar to perform 10 iterations per run_loop()
with 4 samples per run(), so that it wouldn't take all day nor spew pages of
output. The output looks like this:

::

    liar test starting
    test results:
    units are average count of CPU clocks at 13.888889 nanoseconds each
    nop:
        981
        981
        981
        981
    foo:
        101144
        101144
        101144
        101144
    liar test finished

License
-------
stm32f3-liar-test is licensed under the MIT/X11 license.

Portions of the build scripts are from Jorge Aparicio's
cortex-m-quickstart project, copyright © 2017 Jorge
Aparicio, as noted in the particular files. See
https://github.com/japaric/cortex-m-quickstart for
details.

The remainder of stm32f3-liar-test is written by Sean Bolton,
and licensed as follows:

Copyright © 2017 Sean Bolton

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
DEALINGS IN THE SOFTWARE.
