# Carrot graphical engine & Application boilerplate

## TODO

- [ ] GUI
  - [ ] iced carrot gui presets;
- [ ] 2D Engine:
  - [x] Scaling through 2d space;
  - [x] Moving through 2d space;
  - [ ] Geometrical:
    - [ ] Easy CAD-like 2D Geometrical library;
  - [ ] Graphical:
    - [x] Triangulaion of polygons; 
    - [ ] Drawing styled lines;
    - [ ] Drawing styled surface;
  - [ ] Structural:
    - [ ] Layered structure;



### about shape render

Geometric Shape -> Border Path;

if fill -> call triangulate( Border path )

if border -> call triangulate ( call shaper(&Border path, border style, viewport context) -> Border path )

### about 2d world helper structures

#### Layers
