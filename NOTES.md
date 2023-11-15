# Making a game notes

## API

* `Vec3::FORWARD/UP/RIGHT` do not exists? Are useful fot not to thik aboung coordinate system.
* `add_components()` function is very confusing. Is it adding the components of an entity to the provided entity?
Why is it not expecting a list of components instead?

## Components
 
* Water doesn't seem to have other params to control, such as wave steeps, flow speed, look and feel, etc. I couldn't do the project I wanted to. Also, can't create shaders, nor update meshes. So, no piraty game.
* How do I make the sun smaller? Does it even make sense?
* Animation nodes and refs are enities instead of components and they need to be despawned? Very counter intuitive.
* The name of the animations get an automatic `_#` which will be very difficult to predict before hand, hence difficult to create pipeline but for simple demos.
* No navmesh. Point and click is unfeasible.

## Tools

* `--debugger` is broken, it crashes, and it is packed with obscure information hardly useful for a novice, or at all?

## Messages

* Why it doesn't create a Default trait implementation?