# Security Architecture

The MVP is read-only. It launches fixed system tools with fixed argument lists and never executes a shell command assembled from user or hardware data.

Future system modifications must use this shape:

```text
GTK/CLI -> typed operation preview -> D-Bus host helper -> Polkit -> native package manager
```

The helper must expose operations such as `RefreshRepositories`, `InstallDriver(package_id)`, and `RepairDkmsModule(module_id)`. It must not expose `RunAsRoot(command)`.

Every operation must validate identifiers against the selected distribution's package rules, reject paths and shell metacharacters, and return structured success or failure data. The UI must present the current version, proposed version, affected packages, download estimate, and reboot requirement before asking for confirmation.

The Flatpak has no host filesystem access, wildcard D-Bus permissions, or direct package-manager authority. Diagnostic output must omit credentials, tokens, and unrelated personal data.
