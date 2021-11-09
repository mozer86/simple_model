# SIMPLE model


![build badge](https://github.com/SIMPLE-BuildingSimulation/simple_model/actions/workflows/build.yaml/badge.svg)
![docs badge](https://github.com/SIMPLE-BuildingSimulation/simple_model/actions/workflows/docs.yaml/badge.svg)
![tests badge](https://github.com/SIMPLE-BuildingSimulation/simple_model/actions/workflows/tests.yaml/badge.svg)
![ioreference badge](https://github.com/SIMPLE-BuildingSimulation/simple_model/actions/workflows/ioreference.yaml/badge.svg)
![coverage badge](https://github.com/SIMPLE-BuildingSimulation/simple_model/blob/main/coverage/badges/flat.svg)


This repository contains the basic API for creating and 
reading SIMPLE models. This can be used by tools that intend
to help create these models, or by tools that aim to 
use these models for performing simulations (e.g., 
the [Thermal simulation module](https://github.com/SIMPLE-BuildingSimulation/thermal)).

## SIMPLE models from text files

SIMPLE models can be creted through the API (written in RUST programming language), but they can also be written in text files. Check the automatically generated [Input Output reference](https://simple-buildingsimulation.github.io/simple_model/)