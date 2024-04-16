module RingOscillator(stages: int = 5) -> output {
    // Declare internal wiring as an array of wires
    wire [stages-1:0] net;

    // Generate a loop of inverters, forming a ring oscillator
    loop i from 0 to stages - 1 {
        connect not(net[(i + 1) % stages], net[i]);
    }

    // Output from the last inverter stage, which provides the oscillator output
    return net[stages-1];
}

// Define a module to generate random numbers based on a Ring Oscillator
module RandomNumberGenerator(clk: clock, reset: reset) -> output {
    // Instantiate Ring Oscillator and assign its output to unstable_signal
    let unstable_signal = instantiate RingOscillator();

    // Define a register to hold the 8-bit random number
    let random_number: bits(8);

    // Define behavior on the rising edge of the clock
    on rising(clk) {
        if (reset) {
            random_number = 0;  // Reset random_number to 0 when reset is asserted
        } else {
            random_number = concat(random_number[6:0], unstable_signal);  // Shift and append new random bit
        }
    }

    // Return the accumulated random number
    return random_number;
}
