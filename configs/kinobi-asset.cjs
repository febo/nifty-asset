const path = require("path");
const k = require("@metaplex-foundation/kinobi");

// Paths.
const clientDir = path.join(__dirname, "..", "clients");
const idlDir = path.join(__dirname, "..", "idls");

// Instanciate Kinobi.
const kinobi = k.createFromIdls([path.join(idlDir, "asset_program.json")]);

// Update programs.
kinobi.update(
  k.updateProgramsVisitor({
    assetProgram: { name: "asset" },
  })
);

// Add missing types from the IDL.
kinobi.update(
  k.bottomUpTransformerVisitor([
    {
      select: "[programNode]asset",
      transform: (node) => {
        k.assertIsNode(node, "programNode");
        return {
          ...node,
          accounts: [
            ...node.accounts,
            // asset account
            k.accountNode({
              name: "internalAsset",
              data: k.structTypeNode([
                k.structFieldTypeNode({
                  name: "discriminator",
                  type: k.definedTypeLinkNode("Discriminator"),
                }),
                k.structFieldTypeNode({
                  name: "state",
                  type: k.definedTypeLinkNode("State"),
                }),
                k.structFieldTypeNode({
                  name: "standard",
                  type: k.definedTypeLinkNode("Standard"),
                }),
                k.structFieldTypeNode({
                  name: "mutable",
                  type: k.booleanTypeNode(),
                }),
                k.structFieldTypeNode({
                  name: "owner",
                  type: k.publicKeyTypeNode(),
                }),
                k.structFieldTypeNode({
                  name: "group",
                  type: k.definedTypeLinkNode("nullablePublicKey", "hooked"),
                }),
                k.structFieldTypeNode({
                  name: "authority",
                  type: k.publicKeyTypeNode(),
                }),
                k.structFieldTypeNode({
                  name: "delegate",
                  type: k.definedTypeLinkNode("Delegate"),
                }),
                k.structFieldTypeNode({
                  name: "name",
                  type: k.stringTypeNode({ size: k.fixedSizeNode(35) }),
                }),
              ]),
            }),
          ],
          definedTypes: [
            ...node.definedTypes,
            // discriminator
            k.definedTypeNodeFromIdl({
              name: "discriminator",
              type: {
                kind: "enum",
                variants: [{ name: "Uninitialized" }, { name: "Asset" }],
              },
            }),
            // standard
            k.definedTypeNodeFromIdl({
              name: "standard",
              type: {
                kind: "enum",
                variants: [
                  { name: "NonFungible" },
                  { name: "Managed" },
                  { name: "Soulbound" },
                  { name: "Proxied" },
                ],
              },
            }),
            // state
            k.definedTypeNodeFromIdl({
              name: "state",
              type: {
                kind: "enum",
                variants: [{ name: "Unlocked" }, { name: "Locked" }],
              },
            }),
            // delegate role
            k.definedTypeNodeFromIdl({
              name: "delegateRole",
              type: {
                kind: "enum",
                variants: [
                  { name: "None" },
                  { name: "Transfer" },
                  { name: "Lock" },
                  { name: "Burn" },
                ],
              },
            }),
            // extension type
            k.definedTypeNodeFromIdl({
              name: "extensionType",
              type: {
                kind: "enum",
                variants: [
                  { name: "None" },
                  { name: "Attributes" },
                  { name: "Blob" },
                  { name: "Creators" },
                  { name: "Links" },
                  { name: "Metadata" },
                  { name: "Grouping" },
                  { name: "Royalties" },
                  { name: "Manager" },
                  { name: "Proxy" },
                  { name: "Properties" },
                  { name: "Bucket" },
                ],
              },
            }),
            // delegate
            k.definedTypeNode({
              name: "delegate",
              type: k.structTypeNode([
                k.structFieldTypeNode({
                  name: "address",
                  type: k.definedTypeLinkNode("nullablePublicKey", "hooked"),
                }),
                k.structFieldTypeNode({
                  name: "roles",
                  type: k.definedTypeLinkNode("delegateRoles", "hooked"),
                }),
              ]),
            }),
            // extension header
            k.definedTypeNode({
              name: "extensionHeader",
              type: k.structTypeNode([
                k.structFieldTypeNode({
                  name: "kind",
                  type: k.numberTypeNode("u32"),
                }),
                k.structFieldTypeNode({
                  name: "length",
                  type: k.numberTypeNode("u32"),
                }),
                k.structFieldTypeNode({
                  name: "boundary",
                  type: k.numberTypeNode("u32"),
                }),
                k.structFieldTypeNode({
                  name: "padding",
                  type: k.numberTypeNode("u32"),
                }),
              ]),
              internal: true,
            }),
            // attributes
            k.definedTypeNode({
              name: "attributes",
              type: k.structTypeNode([
                k.structFieldTypeNode({
                  name: "values",
                  type: k.arrayTypeNode(
                    k.definedTypeLinkNode("trait"),
                    k.remainderSizeNode()
                  ),
                }),
              ]),
            }),
            // trait
            k.definedTypeNode({
              name: "trait",
              type: k.structTypeNode([
                k.structFieldTypeNode({
                  name: "name",
                  type: k.stringTypeNode({
                    size: k.prefixedSizeNode(k.numberTypeNode("u8")),
                  }),
                }),
                k.structFieldTypeNode({
                  name: "value",
                  type: k.stringTypeNode({
                    size: k.prefixedSizeNode(k.numberTypeNode("u8")),
                  }),
                }),
              ]),
            }),
            // blob
            k.definedTypeNode({
              name: "blob",
              type: k.structTypeNode([
                k.structFieldTypeNode({
                  name: "contentType",
                  type: k.stringTypeNode({
                    size: k.prefixedSizeNode(k.numberTypeNode("u8")),
                  }),
                }),
                k.structFieldTypeNode({
                  name: "data",
                  type: k.arrayTypeNode(
                    k.numberTypeNode("u8"),
                    k.remainderSizeNode()
                  ),
                }),
              ]),
            }),
            // links
            k.definedTypeNode({
              name: "links",
              type: k.structTypeNode([
                k.structFieldTypeNode({
                  name: "values",
                  type: k.arrayTypeNode(
                    k.definedTypeLinkNode("link"),
                    k.remainderSizeNode()
                  ),
                }),
              ]),
            }),
            // link
            k.definedTypeNode({
              name: "link",
              type: k.structTypeNode([
                k.structFieldTypeNode({
                  name: "name",
                  type: k.stringTypeNode({
                    size: k.prefixedSizeNode(k.numberTypeNode("u8")),
                  }),
                }),
                k.structFieldTypeNode({
                  name: "uri",
                  type: k.stringTypeNode({
                    size: k.prefixedSizeNode(k.numberTypeNode("u8")),
                  }),
                }),
              ]),
            }),
            // creators
            k.definedTypeNode({
              name: "creators",
              type: k.structTypeNode([
                k.structFieldTypeNode({
                  name: "values",
                  type: k.arrayTypeNode(
                    k.definedTypeLinkNode("creator"),
                    k.remainderSizeNode()
                  ),
                }),
              ]),
            }),
            // creator
            k.definedTypeNode({
              name: "creator",
              type: k.structTypeNode([
                k.structFieldTypeNode({
                  name: "address",
                  type: k.publicKeyTypeNode(),
                }),
                k.structFieldTypeNode({
                  name: "verified",
                  type: k.booleanTypeNode(),
                }),
                k.structFieldTypeNode({
                  name: "share",
                  type: k.numberTypeNode("u8"),
                }),
              ]),
            }),
            // metadata
            k.definedTypeNode({
              name: "metadata",
              type: k.structTypeNode([
                k.structFieldTypeNode({
                  name: "symbol",
                  type: k.stringTypeNode({
                    size: k.prefixedSizeNode(k.numberTypeNode("u8")),
                  }),
                }),
                k.structFieldTypeNode({
                  name: "description",
                  type: k.stringTypeNode({
                    size: k.prefixedSizeNode(k.numberTypeNode("u8")),
                  }),
                }),
                k.structFieldTypeNode({
                  name: "uri",
                  type: k.stringTypeNode({
                    size: k.prefixedSizeNode(k.numberTypeNode("u8")),
                  }),
                }),
                k.structFieldTypeNode({
                  name: "imageUri",
                  type: k.stringTypeNode({
                    size: k.prefixedSizeNode(k.numberTypeNode("u8")),
                  }),
                }),
              ]),
            }),
            // grouping
            k.definedTypeNode({
              name: "grouping",
              type: k.structTypeNode([
                k.structFieldTypeNode({
                  name: "size",
                  type: k.numberTypeNode("u64"),
                }),
                k.structFieldTypeNode({
                  name: "maxSize",
                  type: k.numberTypeNode("u64"),
                }),
                k.structFieldTypeNode({
                  name: "delegate",
                  type: k.definedTypeLinkNode("nullablePublicKey", "hooked"),
                }),
              ]),
            }),
            // manager
            k.definedTypeNode({
              name: "manager",
              type: k.structTypeNode([
                k.structFieldTypeNode({
                  name: "delegate",
                  type: k.definedTypeLinkNode("delegate"),
                }),
              ]),
            }),
            // proxy
            k.definedTypeNode({
              name: "proxy",
              type: k.structTypeNode([
                k.structFieldTypeNode({
                  name: "program",
                  type: k.publicKeyTypeNode(),
                }),
                k.structFieldTypeNode({
                  name: "seeds",
                  type: k.arrayTypeNode(
                    k.numberTypeNode("u8"),
                    k.fixedSizeNode(32)
                  ),
                }),
                k.structFieldTypeNode({
                  name: "bump",
                  type: k.numberTypeNode("u8"),
                }),
                k.structFieldTypeNode({
                  name: "authority",
                  type: k.definedTypeLinkNode("nullablePublicKey", "hooked"),
                }),
              ]),
            }),
            // bucket
            k.definedTypeNode({
              name: "bucket",
              type: k.structTypeNode([
                k.structFieldTypeNode({
                  name: "data",
                  type: k.arrayTypeNode(
                    k.numberTypeNode("u8"),
                    k.remainderSizeNode()
                  ),
                }),
              ]),
            }),
            // type (for properties extension)
            k.definedTypeNodeFromIdl({
              name: "type",
              type: {
                kind: "enum",
                variants: [
                  { name: "Text" },
                  { name: "Number" },
                  { name: "Boolean" },
                ],
              },
            }),
          ],
        };
      },
    },
  ])
);

// Update instructions.
kinobi.update(
  k.updateInstructionsVisitor({
    allocate: {
      accounts: {
        systemProgram: {
          defaultValue: k.conditionalValueNode({
            condition: k.accountValueNode("payer"),
            ifTrue: k.publicKeyValueNode(
              "11111111111111111111111111111111",
              "systemProgram"
            ),
          }),
        },
      },
    },
    approve: {
      accounts: {
        owner: { defaultValue: k.identityValueNode() },
      },
    },
    create: {
      accounts: {
        owner: { defaultValue: k.identityValueNode() },
        authority: { defaultValue: k.identityValueNode() },
        systemProgram: {
          defaultValue: k.conditionalValueNode({
            condition: k.accountValueNode("payer"),
            ifTrue: k.publicKeyValueNode(
              "11111111111111111111111111111111",
              "systemProgram"
            ),
          }),
        },
      },
      arguments: {
        extensions: {
          defaultValue: k.noneValueNode(),
        },
      },
    },
    handover: {
      accounts: {
        authority: { defaultValue: k.identityValueNode() },
      },
    },
    group: {
      accounts: {
        authority: { defaultValue: k.identityValueNode() },
      },
    },
    lock: {
      accounts: {
        signer: { defaultValue: k.identityValueNode() },
      },
    },
    remove: {
      accounts: {
        authority: { defaultValue: k.identityValueNode() },
        recipient: { defaultValue: k.identityValueNode() },
      },
    },
    resize: {
      accounts: {
        authority: { defaultValue: k.identityValueNode() },
        systemProgram: {
          defaultValue: k.conditionalValueNode({
            condition: k.accountValueNode("payer"),
            ifTrue: k.publicKeyValueNode(
              "11111111111111111111111111111111",
              "systemProgram"
            ),
          }),
        },
      },
    },
    revoke: {
      accounts: {
        signer: { defaultValue: k.identityValueNode() },
      },
    },
    transfer: {
      accounts: {
        signer: { defaultValue: k.identityValueNode() },
      },
    },
    ungroup: {
      accounts: {
        authority: { defaultValue: k.identityValueNode() },
      },
    },
    unlock: {
      accounts: {
        signer: { defaultValue: k.identityValueNode() },
      },
    },
    update: {
      accounts: {
        authority: { defaultValue: k.identityValueNode() },
        systemProgram: {
          defaultValue: k.conditionalValueNode({
            condition: k.accountValueNode("payer"),
            ifTrue: k.publicKeyValueNode(
              "11111111111111111111111111111111",
              "systemProgram"
            ),
          }),
        },
      },
    },
    write: {
      accounts: {
        systemProgram: {
          defaultValue: k.publicKeyValueNode(
            "11111111111111111111111111111111",
            "systemProgram"
          ),
        },
      },
    },
  })
);

// Set more struct default values dynamically.
kinobi.update(
  k.bottomUpTransformerVisitor([
    {
      select: "[instructionNode|instructionArgumentNode]standard",
      transform: (node) => {
        k.assertIsNode(node, ["instructionNode", "instructionArgumentNode"]);
        return {
          ...node,
          defaultValue: k.enumValueNode("Standard", "NonFungible"),
        };
      },
    },
    {
      select: "[instructionNode|instructionArgumentNode]mutable",
      transform: (node) => {
        k.assertIsNode(node, ["instructionNode", "instructionArgumentNode"]);
        return {
          ...node,
          defaultValue: k.booleanValueNode(true),
        };
      },
    },
    {
      select: (node) => {
        const names = ["name", "mutable", "extension"];
        return (
          k.isNode(node, ["instructionNode", "instructionArgumentNode"]) &&
          k.isNode(node.type, "optionTypeNode") &&
          names.includes(node.name)
        );
      },
      transform: (node) => {
        k.assertIsNode(node, ["instructionNode", "instructionArgumentNode"]);
        return {
          ...node,
          defaultValueStrategy: "optional",
          defaultValue: k.noneValueNode(),
        };
      },
    },
  ])
);

// Set ShankAccount discriminator.
const key = (name) => ({
  field: "discriminator",
  value: k.enumValueNode("Discriminator", name),
});
kinobi.update(
  new k.setAccountDiscriminatorFromFieldVisitor({
    Asset: key("Asset"),
  })
);

// Render JavaScript.
kinobi.accept(
  new k.renderJavaScriptVisitor(
    path.join(clientDir, "js", "asset", "src", "generated"),
    {
      prettier: require(path.join(
        clientDir,
        "js",
        "asset",
        ".prettierrc.json"
      )),
      internalNodes: [
        "allocate",
        "approve",
        "burn",
        "create",
        "group",
        "handover",
        "lock",
        "remove",
        "resize",
        "revoke",
        "transfer",
        "ungroup",
        "unlock",
        "unverify",
        "update",
        "verify",
        "write",
        "internalAsset",
      ],
      customAccountData: [
        {
          name: "internalAsset",
          extract: true,
        },
      ],
    }
  )
);

// Render Rust.
kinobi.accept(
  new k.renderRustVisitor(
    path.join(clientDir, "rust", "asset", "src", "generated"),
    {
      formatCode: true,
      crateFolder: path.join(clientDir, "rust", "asset"),
    }
  )
);
