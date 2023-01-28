# Magitian Git

A Git Smart Http implementation written in rust to be used for the Magitian git service.
This crate handles interaction with git and communticates with other services to form the Magitian web service.
Magitian Git does not actually interact with the repositories it instead sends requests to another service called <NOT_YET_CREATED>
that actually executes the operation such as:
- advertising refs
- packfiles
- branch creation
- etc...

## Contributing

If you are interested in contributing then please consult [CONTRIBUTING](CONTRIBUTING.md)

## Stages for now

| stage-1 | stage-2 |
|---------|---------|
| write a full implementation that has the correct responses for all cases. | Intergrate Auth into the Magitian SHELL |
| authentication | Intergrate commit tracking into Magition SHELL |
