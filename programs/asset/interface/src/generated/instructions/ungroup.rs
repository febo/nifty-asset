//! This code was AUTOGENERATED using the kinobi library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun kinobi to update it.
//!
//! [https://github.com/metaplex-foundation/kinobi]
//!

use borsh::BorshDeserialize;
use borsh::BorshSerialize;

/// Accounts.
pub struct Ungroup {
    /// Asset account
    pub asset: solana_program::pubkey::Pubkey,
    /// Asset account of the group
    pub group: solana_program::pubkey::Pubkey,
    /// The authority of the assets
    pub authority: solana_program::pubkey::Pubkey,
}

impl Ungroup {
    pub fn instruction(&self) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(&[])
    }
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(3 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.asset, true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.group, false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.authority,
            true,
        ));
        accounts.extend_from_slice(remaining_accounts);
        let data = UngroupInstructionData::new().try_to_vec().unwrap();

        solana_program::instruction::Instruction {
            program_id: crate::INTERFACE_ID,
            accounts,
            data,
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize)]
struct UngroupInstructionData {
    discriminator: u8,
}

impl UngroupInstructionData {
    fn new() -> Self {
        Self { discriminator: 14 }
    }
}

/// Instruction builder for `Ungroup`.
///
/// ### Accounts:
///
///   0. `[writable, signer]` asset
///   1. `[writable]` group
///   2. `[signer]` authority
#[derive(Default)]
pub struct UngroupBuilder {
    asset: Option<solana_program::pubkey::Pubkey>,
    group: Option<solana_program::pubkey::Pubkey>,
    authority: Option<solana_program::pubkey::Pubkey>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl UngroupBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    /// Asset account
    #[inline(always)]
    pub fn asset(&mut self, asset: solana_program::pubkey::Pubkey) -> &mut Self {
        self.asset = Some(asset);
        self
    }
    /// Asset account of the group
    #[inline(always)]
    pub fn group(&mut self, group: solana_program::pubkey::Pubkey) -> &mut Self {
        self.group = Some(group);
        self
    }
    /// The authority of the assets
    #[inline(always)]
    pub fn authority(&mut self, authority: solana_program::pubkey::Pubkey) -> &mut Self {
        self.authority = Some(authority);
        self
    }
    /// Add an aditional account to the instruction.
    #[inline(always)]
    pub fn add_remaining_account(
        &mut self,
        account: solana_program::instruction::AccountMeta,
    ) -> &mut Self {
        self.__remaining_accounts.push(account);
        self
    }
    /// Add additional accounts to the instruction.
    #[inline(always)]
    pub fn add_remaining_accounts(
        &mut self,
        accounts: &[solana_program::instruction::AccountMeta],
    ) -> &mut Self {
        self.__remaining_accounts.extend_from_slice(accounts);
        self
    }
    #[allow(clippy::clone_on_copy)]
    pub fn instruction(&self) -> solana_program::instruction::Instruction {
        let accounts = Ungroup {
            asset: self.asset.expect("asset is not set"),
            group: self.group.expect("group is not set"),
            authority: self.authority.expect("authority is not set"),
        };

        accounts.instruction_with_remaining_accounts(&self.__remaining_accounts)
    }
}

/// `ungroup` CPI accounts.
pub struct UngroupCpiAccounts<'a, 'b> {
    /// Asset account
    pub asset: &'b solana_program::account_info::AccountInfo<'a>,
    /// Asset account of the group
    pub group: &'b solana_program::account_info::AccountInfo<'a>,
    /// The authority of the assets
    pub authority: &'b solana_program::account_info::AccountInfo<'a>,
}

/// `ungroup` CPI instruction.
pub struct UngroupCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,
    /// Asset account
    pub asset: &'b solana_program::account_info::AccountInfo<'a>,
    /// Asset account of the group
    pub group: &'b solana_program::account_info::AccountInfo<'a>,
    /// The authority of the assets
    pub authority: &'b solana_program::account_info::AccountInfo<'a>,
}

impl<'a, 'b> UngroupCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: UngroupCpiAccounts<'a, 'b>,
    ) -> Self {
        Self {
            __program: program,
            asset: accounts.asset,
            group: accounts.group,
            authority: accounts.authority,
        }
    }
    #[inline(always)]
    pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(&[], &[])
    }
    #[inline(always)]
    pub fn invoke_with_remaining_accounts(
        &self,
        remaining_accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(&[], remaining_accounts)
    }
    #[inline(always)]
    pub fn invoke_signed(
        &self,
        signers_seeds: &[&[&[u8]]],
    ) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(signers_seeds, &[])
    }
    #[allow(clippy::clone_on_copy)]
    #[allow(clippy::vec_init_then_push)]
    pub fn invoke_signed_with_remaining_accounts(
        &self,
        signers_seeds: &[&[&[u8]]],
        remaining_accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> solana_program::entrypoint::ProgramResult {
        let mut accounts = Vec::with_capacity(3 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.asset.key,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.group.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.authority.key,
            true,
        ));
        remaining_accounts.iter().for_each(|remaining_account| {
            accounts.push(solana_program::instruction::AccountMeta {
                pubkey: *remaining_account.0.key,
                is_signer: remaining_account.1,
                is_writable: remaining_account.2,
            })
        });
        let data = UngroupInstructionData::new().try_to_vec().unwrap();

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::INTERFACE_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(3 + 1 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.asset.clone());
        account_infos.push(self.group.clone());
        account_infos.push(self.authority.clone());
        remaining_accounts
            .iter()
            .for_each(|remaining_account| account_infos.push(remaining_account.0.clone()));

        if signers_seeds.is_empty() {
            solana_program::program::invoke(&instruction, &account_infos)
        } else {
            solana_program::program::invoke_signed(&instruction, &account_infos, signers_seeds)
        }
    }
}

/// Instruction builder for `Ungroup` via CPI.
///
/// ### Accounts:
///
///   0. `[writable, signer]` asset
///   1. `[writable]` group
///   2. `[signer]` authority
pub struct UngroupCpiBuilder<'a, 'b> {
    instruction: Box<UngroupCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> UngroupCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(UngroupCpiBuilderInstruction {
            __program: program,
            asset: None,
            group: None,
            authority: None,
            __remaining_accounts: Vec::new(),
        });
        Self { instruction }
    }
    /// Asset account
    #[inline(always)]
    pub fn asset(&mut self, asset: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.asset = Some(asset);
        self
    }
    /// Asset account of the group
    #[inline(always)]
    pub fn group(&mut self, group: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.group = Some(group);
        self
    }
    /// The authority of the assets
    #[inline(always)]
    pub fn authority(
        &mut self,
        authority: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.authority = Some(authority);
        self
    }
    /// Add an additional account to the instruction.
    #[inline(always)]
    pub fn add_remaining_account(
        &mut self,
        account: &'b solana_program::account_info::AccountInfo<'a>,
        is_writable: bool,
        is_signer: bool,
    ) -> &mut Self {
        self.instruction
            .__remaining_accounts
            .push((account, is_writable, is_signer));
        self
    }
    /// Add additional accounts to the instruction.
    ///
    /// Each account is represented by a tuple of the `AccountInfo`, a `bool` indicating whether the account is writable or not,
    /// and a `bool` indicating whether the account is a signer or not.
    #[inline(always)]
    pub fn add_remaining_accounts(
        &mut self,
        accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> &mut Self {
        self.instruction
            .__remaining_accounts
            .extend_from_slice(accounts);
        self
    }
    #[inline(always)]
    pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed(&[])
    }
    #[allow(clippy::clone_on_copy)]
    #[allow(clippy::vec_init_then_push)]
    pub fn invoke_signed(
        &self,
        signers_seeds: &[&[&[u8]]],
    ) -> solana_program::entrypoint::ProgramResult {
        let instruction = UngroupCpi {
            __program: self.instruction.__program,

            asset: self.instruction.asset.expect("asset is not set"),

            group: self.instruction.group.expect("group is not set"),

            authority: self.instruction.authority.expect("authority is not set"),
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

struct UngroupCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    asset: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    group: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    authority: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}
