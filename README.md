# Tiny Render in Rust
Following [https://github.com/ssloy/tinyrenderer/wiki](https://github.com/ssloy/tinyrenderer/wiki) .

#### Thanks
[@jonvaldes](https://github.com/jonvaldes) for [TGA file editor](https://gist.github.com/jonvaldes/607fbc380f816d205afb).

## Start

Here is the first attempt.

Let us split up two coordinate to just x and just y. We now have two constant: $$x = x_0 + (x_1 - x_0)$$ and $$y = y_0 +(y_1 - y_0)$$. It is easy to find that $x_1 - x_0$ means the distance from $$x_0$$ to $$x_1$$, $$y_1 - y_0$$ got the same meaning.

So we add up $$x_0$$(I call it the base) and $$x_1 - x_0$$, we got $$x$$; Same to the $$y$$.

In this attempt, we split the line(or the distance above) into 100 points(0..1 step=0.01).

```rust
fn draw_line(self: &mut Self, x0: u32, y0: u32, x1: u32, y1: u32, color: Color) {
    let mut t= 0f32;
    while t < 1f32 {
        let x = (x0 + (x1 - x0)) as f32 * t;
        let y = (y0 + (y1 - y0)) as f32 * t;
        self.set_pixel(x as u32, y as u32, color.clone());
        t += 0.01f32
    }
}
```

![First Attempt](https://cdn.jsdelivr.net/gh/DarcJC/pictures-host/imgs/20210216175055.png)

There will be more gaps if we change `t += 0.01f32` to `t += 0.1f32`.