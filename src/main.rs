use hello_wgpu::run;

fn main() {
    // TODO it would be nice if I could get this working with tokio, but mio is
    // complaining so this pollster thing will have to do for now.
    pollster::block_on(run());
}
