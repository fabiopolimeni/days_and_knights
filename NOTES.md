# Making a game notes

## API

* `Vec3::FORWARD/UP/RIGHT` do not exists? Are useful fot not to think aboung coordinate system. Maybe should be a Pos2/3?
* Would be nice to get more out of the Transform
* `add_components()` function is very confusing. Is it adding the components of an entity to the provided entity?,Why is it not expecting a list of components?
* No CharacterController specific documentation, and a lot of other components and concepts. I have the feeling the TOML file declaration does not feel right for documentation.
* Why messages don't have default `Default` trait implementation?
* Why I don't get the Collision::subscribe API from the `physics` documentation, where is it? Oh, it is under messages namespace It should go under `physics::messages` the same way we have `physics::components/concepts`.
* `client_entity_id()`, why it is not really `client_player_id()`?
* Why Collision and Frame are messages? I understand we use messages through packages, but now the server needs to subscribe to messages to receive collision events? (maybe ok).
* Confusing is the fact that, within the `core` module, there exist modules named the same as some of the outer ones, such as `animation`, `physics` etc. It seems that `core` really is a container of all other module components and concepts.
* It looks like there is a bit of mismatching on the concept of entities and transformations. Parent/children can still be ok, but `in_area` and `get_transformers_relative_to` look misplaced. Also, it gives the chance to do the same thing through different APIs, which will contribute to API fragmentation.
* Can't load GLB files from URL?
* Why all these *Optional thingy? Are they really needed, what is the rationale? Why would I want to pay, if nothing else, memory for unused components?
* This `animation` API is very biased and opinionated towards humanoid skeleton structures. At least we should have a utility function such `animaiton::get_bone_by_name/index()`.

## Components
 
* The water component doesn't seem to have other params to control, such as wave steeps, flow speed, look and feel, etc. I couldn't do the project I wanted to. Also, can't create shaders, nor update meshes. So, no piraty game.
* No navmesh. Point and click games are unfeasible.
* How do I make the sun smaller? Does it even make sense?
* Animation nodes and refs are enities instead of components and they need to be despawned? Very counter intuitive.
* The name of the animations get an automatic `_#` which makes difficult to predict before hand, hence create CI pipelines but for simple demos.
* I feel I should have used the `animation_controller`, but the lack of documentation made me opt for writing a simpler animator, rather than spending time learning an undocumented component.
* Why the AnimationPlayer element is an UI thingy? Also it would have been nice to get an example of how to use it.
* Why I don't have a `capsule_collision()` component?

## Tools

* `--debugger` is broken, it crashes, and it is packed with obscure information hardly useful for a novice, or at all, and impossible to inspect?
* Online Rust Ambient API documentation keeps returning 404 error. I guess it happens every time the documentation is uploaded, even the URL simply points to latest. Weird though.

## Bugs

* It seems I am not able to load GLTF and FBX files.
