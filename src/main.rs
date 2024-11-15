use std::time::Duration;

use num_bigint::BigUint;
use rust_hdl::prelude::*;

#[derive(LogicBlock)]
struct SevenSegmentDisplay {
    clock: Signal<In, Clock>,
    pulser: Pulser,

    // mem
    internal_count: DFF<Bits<4>>,
    segment_state: Signal<Local, Bits<7>>,

    // input
    i_switch_1: Signal<In, Bit>,
    // debouncing
    button_prev: DFF<Bit>,

    // outputs
    o_led_1: Signal<Out, Bit>,

    o_segment1_a: Signal<Out, Bit>,
    o_segment1_b: Signal<Out, Bit>,
    o_segment1_c: Signal<Out, Bit>,
    o_segment1_d: Signal<Out, Bit>,
    o_segment1_e: Signal<Out, Bit>,
    o_segment1_f: Signal<Out, Bit>,
    o_segment1_g: Signal<Out, Bit>,

    mapping: Constant<Bits<70>>,
}

const CLOCK_SPEED_HZ: u64 = 25_000_000;
impl Default for SevenSegmentDisplay {
    fn default() -> Self {
        let mapping = {
            //   ABCDEFG
            let segments: [_; 10] = [
                "1111110", // 0
                "0110000", // 1
                "1101101", // 2
                "1111001", // 3
                "0110011", // 4
                "1011011", // 5
                "1011111", // 6
                "1110000", // 7
                "1111111", // 8
                "1111011", // 9
            ];

            Constant::new(
                BigUint::parse_bytes(
                    segments
                        .into_iter()
                        .collect::<String>()
                        .chars()
                        .rev()
                        // flip
                        .map(|c| if c == '1' { '0' } else { '1' })
                        .collect::<String>()
                        .as_bytes(),
                    2,
                )
                .unwrap()
                .into(),
            )
        };

        Self {
            clock: Default::default(),
            pulser: Pulser::new(CLOCK_SPEED_HZ, 0.2, Duration::from_millis(100)),
            internal_count: Default::default(),
            segment_state: Default::default(),
            button_prev: Default::default(),
            o_led_1: Default::default(),
            i_switch_1: Default::default(),
            o_segment1_a: Default::default(),
            o_segment1_b: Default::default(),
            o_segment1_c: Default::default(),
            o_segment1_d: Default::default(),
            o_segment1_e: Default::default(),
            o_segment1_f: Default::default(),
            o_segment1_g: Default::default(),
            mapping,
        }
    }
}

impl Logic for SevenSegmentDisplay {
    #[hdl_gen]
    fn update(&mut self) {
        // Clocks
        self.internal_count.clock.next = self.clock.val();
        self.button_prev.clock.next = self.clock.val();
        self.pulser.clock.next = self.clock.val();

        self.pulser.enable.next = true;

        self.o_led_1.next = !self.pulser.pulse.val();

        self.internal_count.d.next = self.internal_count.q.val();

        if (self.pulser.pulse.val() | self.i_switch_1.val()) && !self.button_prev.q.val() {
            if self.internal_count.q.val() > 8 {
                self.internal_count.d.next = 0.into();
            } else {
                self.internal_count.d.next = self.internal_count.q.val() + 1u8.to_bits();
            }
        }

        self.button_prev.d.next = self.pulser.pulse.val() | self.i_switch_1.val();

        self.segment_state.next = self
            .mapping
            .val()
            .get_bits::<7>(self.internal_count.q.val().index() * 7);
        self.o_segment1_a.next = self.segment_state.val().get_bit(0);
        self.o_segment1_b.next = self.segment_state.val().get_bit(1);
        self.o_segment1_c.next = self.segment_state.val().get_bit(2);
        self.o_segment1_d.next = self.segment_state.val().get_bit(3);
        self.o_segment1_e.next = self.segment_state.val().get_bit(4);
        self.o_segment1_f.next = self.segment_state.val().get_bit(5);
        self.o_segment1_g.next = self.segment_state.val().get_bit(6);
    }
}

fn main() {
    let mut display = SevenSegmentDisplay::default();

    display.connect_all();
    println!("{}", generate_verilog(&display));
}
