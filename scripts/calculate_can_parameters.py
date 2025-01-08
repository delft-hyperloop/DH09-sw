import math
import matplotlib.pyplot as plt

# Constantas related to your device (check reference manual / datasheet)
MIN_TIME_QUANTA = 4
MAX_TIME_QUANTA_NOMINAL = 81
MAX_TIME_QUANTA_BIT = 65
IPT = 0

MAX_SEG1_NOMINAL = 256
MAX_SEG2_NOMINAL = 128

MAX_SEG1_BIT = 32
MAX_SEG2_BIT = 16

def oscillator_tolerance(prop_seg, phase_seg1, phase_seg2, sjw):
    bit_time = 1 + prop_seg + phase_seg1 + phase_seg2

    df1 = min(phase_seg1, phase_seg2) / (2 * (13 * bit_time - phase_seg2))
    df2 = sjw / (20 * bit_time)

    return min(df1, df2)

clk_frequencies = [i * 10**6 for i in range(20, 130, 10)] # Hz

nominal_bitrate = 1000 * 10**3
fd_bitrate = 2000 * 10**3

### Propagation delay calculations
TRANSCEIVER_LOOP_DELAY = 200 * 10**-9
CABLE_LENGTH = 1#m

propagation_delay = (5*10**-9) * CABLE_LENGTH + TRANSCEIVER_LOOP_DELAY # One way propagation delay
###

# Returns tuple (brp, prop_seg, phase_seg1, phase_seg2, sjw, tolerance)
def calculate_nominal_parameters(min_time_quanta, max_time_quanta, 
                                 max_seg1, max_seg2,
                                 prop_delay, 
                                 rate, clk_frequency):
    best_parameters = None

    for quantas in range(min_time_quanta, max_time_quanta + 1):
        # t_q = brp / f_osc
        # bitrate = 1 / (t_q * quantas)
        # brp = f_osc / (bitrate * quantas)
        if clk_frequency % (rate * quantas) != 0:
            continue

        # Calculate BRP to achieve bitrate with given quantas
        brp = clk_frequency / (rate * quantas)

        # Time quantum of bit timings
        tq = brp / clk_frequency

        # Minimum propgation segment quantas given one way delay
        prop_seg = math.ceil(prop_delay / tq)

        if prop_seg < 1:
            continue

        phase_quantas = quantas - 1 - prop_seg

        phase_seg1 = math.floor(phase_quantas / 2)
        phase_seg2 = phase_quantas - phase_seg1

        # phase_seg2 is less than 2 to guarantee that it can accomodate the information processing time (IPT)
        if phase_seg1 < 1 or phase_seg2 < max(1, IPT) or (phase_seg1 + prop_seg) > max_seg1 or phase_seg2 > max_seg2:
            continue

        assert (1 + phase_seg1 + phase_seg2 + prop_seg == quantas)

        sjw = min(4, phase_seg1)

        df = oscillator_tolerance(prop_seg, phase_seg1, phase_seg2, sjw)

        if best_parameters is None or df > best_parameters[5]:
            best_parameters = (brp, prop_seg, phase_seg1, phase_seg2, sjw, df)
    
    return best_parameters

# Similar to nominal, but with different constraints:
# 1. We want to share the tq or the nominal timing, so we pass that in
# 2. We can ignore propagation time since when sending a bunch of data, the receivers will adjust to the propagation delay
# Returns tuple (phase_seg1, phase_seg2, sjw, tolerance)
def calculate_bit_parameters(min_time_quanta, max_time_quanta,
                             max_seg1, max_seg2, 
                             rate, clk_frequency):
    
    best_parameters = None

    for quantas in range(min_time_quanta, max_time_quanta + 1):
        # t_q = brp / f_osc
        # bitrate = 1 / (t_q * quantas)
        # brp = f_osc / (bitrate * quantas)
        if clk_frequency % (rate * quantas) != 0:
            continue

        # Calculate BRP to achieve bitrate with given quantas
        brp = clk_frequency / (rate * quantas)

        # Time quantum of bit timings
        tq = brp / clk_frequency

        # During the data transfer we only do 1 way communication, so synchronization aligns receiving nodes to the propagation delay 
        prop_seg = 0
    
        phase_quantas = quantas - 1 - prop_seg

        phase_seg1 = math.floor(phase_quantas / 2) # Sample point is close to 50% for FD
        phase_seg2 = phase_quantas - phase_seg1

        # phase_seg2 must accomodate the information propagation delay of the CAN peripheral
        # For the STM32H4xx series, we have the newer FDCAN peripherals with 0 ITM delay. BxCAN peripherals may differ.
        if phase_seg1 < 1 or phase_seg2 < max(1, IPT) or (phase_seg1 + prop_seg) > max_seg1 or phase_seg2 > max_seg2:
            continue

        assert (1 + phase_seg1 + phase_seg2 + prop_seg == quantas)

        sjw = min(4, phase_seg1)

        df = oscillator_tolerance(prop_seg, phase_seg1, phase_seg2, sjw)

        if best_parameters is None or df > best_parameters[4]:
            best_parameters = (brp, phase_seg1, phase_seg2, sjw, df)
    
    return best_parameters

plt_freq = []
plt_tol = []

for clk_freq in clk_frequencies:
    best_nominal = calculate_nominal_parameters(MIN_TIME_QUANTA, MAX_TIME_QUANTA_NOMINAL, 
                                        MAX_SEG1_NOMINAL, MAX_SEG2_NOMINAL,
                                        propagation_delay * 2,
                                        nominal_bitrate, clk_freq)
    
    if best_nominal is None:
        continue

    best_bit = calculate_bit_parameters(MIN_TIME_QUANTA, MAX_TIME_QUANTA_BIT,
                                        MAX_SEG1_BIT, MAX_SEG2_BIT,
                                        fd_bitrate, clk_freq)
    
    if best_bit is None:
        continue

    minimum_tolerance = math.floor(min(best_nominal[5], best_bit[3])*100*1e4)
    plt_freq.append(clk_freq * 1e-6)
    plt_tol.append(minimum_tolerance)

    print(f"Frequency: {clk_freq*1e-6:.0f}MHz")
    print(f"Tolerance: {minimum_tolerance}ppm")
    print(
        f"""Configuration nominal:
            BRP: {best_nominal[0]}
            prop_seg: {best_nominal[1]}tq
            phase_seg1: {best_nominal[2]}tq
            phase_seg2: {best_nominal[3]}tq
            sjw: {best_nominal[4]}tq
            tolerance: {best_nominal[5] * 100:0.3f}%
            sample pont: {100*(best_nominal[1] + best_nominal[2] + 1)/(best_nominal[1] + best_nominal[2] + best_nominal[3] + 1):.1f}%
            """)
    
    print(
        f"""Configuration bit:
            BRP: {best_bit[0]}
            phase_seg1: {best_bit[1]}tq
            phase_seg2: {best_bit[2]}tq
            sjw: {best_bit[3]}tq
            tolerance: {best_bit[4] * 100:0.3f}%
            sample pont: {100*(best_bit[1] + 1)/(best_bit[1] + best_bit[2] + 1):.1f}%
            """)

plt.title(f"Oscillator tolerance w.r.t kernel clock frequency @ {CABLE_LENGTH}m")
plt.xlabel("Frequency (MHz)")
plt.ylabel("Tolerance (ppm = 0.0001%)")
plt.axhline(0.5 * 10000, c="red", label="HCI (25°C)")
plt.legend(loc="lower right")
plt.scatter(plt_freq, plt_tol)

for freq, tol in zip(plt_freq, plt_tol): plt.annotate(f"{freq:.0f}MHz", (freq-1, tol-500), fontsize="small", rotation="vertical")

plt.show()