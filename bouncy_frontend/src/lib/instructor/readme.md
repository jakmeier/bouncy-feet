bouncy_instructor_bg.wasm needs to be built and copied here.

```
cd bouncy_instructor
make
```

For remote hosting, due to vite-plugin-wasm-esm being the only thing that works
with svelte 5 and wasm but generates absolute local paths, we have to bundle it
on the actual host. In that case, copy the wasm in this folder from a machine
where it can be built from source, if the host has not a full development setup.