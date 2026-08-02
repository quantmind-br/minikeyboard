# Security policy

## Reporting a vulnerability

Use GitHub's private vulnerability reporting for this repository. Avoid public
issues for vulnerabilities that could write an unsafe device configuration,
execute an unintended command, or expose captured input.

## Hardware safety

Unknown and experimental devices are read-only. Write support requires an
explicitly validated device tuple and reversible write/readback evidence. Do
not weaken these checks to add nominal device compatibility.

The launcher daemon executes commands configured by the local user. Review
imported profiles before enabling the service and keep the bindings file
writable only by that user.
