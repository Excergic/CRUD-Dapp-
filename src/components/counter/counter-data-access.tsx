'use client'

import { getCounterProgram, getCounterProgramId } from '@project/anchor'
import { useConnection } from '@solana/wallet-adapter-react'
import { Cluster, Keypair, PublicKey, SystemProgram } from '@solana/web3.js'
import { useMutation, useQuery } from '@tanstack/react-query'
import { useMemo } from 'react'
import { useCluster } from '../cluster/cluster-data-access'
import { useAnchorProvider } from '../solana/solana-provider'
import { useTransactionToast } from '../use-transaction-toast'
import { toast } from 'sonner'
import * as anchor from '@coral-xyz/anchor'

interface CreateEntryArgs {
  title: string,
  message: string,
  owner: PublicKey,
}

export function useCounterProgram() {
  const { connection } = useConnection()
  const { cluster } = useCluster()
  const transactionToast = useTransactionToast()
  const provider = useAnchorProvider()
   const programId = useMemo(() => {

    if (cluster.network === 'devnet') {
      return new PublicKey('21NReLiauM9EUhYMKoCjf6rZrWJhHZUQaPnWHkRrVAJb')
    }
    return getCounterProgramId(cluster.network as Cluster)
  }, [cluster])
  const program = useMemo(() => getCounterProgram(provider, programId), [provider, programId])

  const accounts = useQuery({
    queryKey: ['counter', 'all', { cluster }],
    queryFn: () => program.account.journalEntryState.all(),
  })

  const getProgramAccount = useQuery({
    queryKey: ['get-program-account', { cluster }],
    queryFn: () => connection.getParsedAccountInfo(programId),
  })

  const createEntry = useMutation<string, Error, CreateEntryArgs>({
    mutationKey : [`journalEntry`, `create`, { cluster }],
    mutationFn: async ({ title, message, owner }) => {
      const [journalEntryPda] = PublicKey.findProgramAddressSync(
        [Buffer.from(title), owner.toBuffer()],
        programId
      );

      const [userProfilePda] = PublicKey.findProgramAddressSync(
        [Buffer.from('user_profile'), owner.toBuffer()],
        programId
      );

      return program.methods
        .createJournalEntry(title, message)
        .accounts({
          journalEntry: journalEntryPda,
          userProfile: userProfilePda,
          owner: owner,
          systemProgram: SystemProgram.programId,
          clock: anchor.web3.SYSVAR_CLOCK_PUBKEY  
        })
        .rpc();
    },

    onSuccess: (signature) => {
      transactionToast(signature);
      accounts.refetch();
    },

    onError: (error) => {
      toast.error(`Error creating journal entry: ${error.message}`);
    },
  });

  return {
    program,
    programId,
    accounts,
    getProgramAccount,
    createEntry,
  }
}

export function useCounterProgramAccount({ account }: { account: PublicKey }) {
  const { cluster } = useCluster()
  const transactionToast = useTransactionToast()
  const { program, accounts } = useCounterProgram()

  const accountQuery = useQuery({
    queryKey: ['counter', 'fetch', { cluster, account }],
    queryFn: () => program.account.journalEntryState.fetch(account),
  })
  


  const updateEntry = useMutation<string, Error, CreateEntryArgs> ({
    mutationKey: [`journlEntry`, `update`, { cluster }],
    mutationFn: async({ title, message }) => {
      return program.methods.updateJournalEntry(title, message).rpc();
    },

    onSuccess: (sinature) => {
      transactionToast(sinature);
      accounts.refetch();
    },

    onError: (error) => {
      toast.error(`Error updating journal entry: ${error.message}`);
    },
  });

  const deleteEntry = useMutation({
    mutationKey: [`journalEntry`, `delete`, { cluster }],
    mutationFn: (title: string) => {
      return program.methods.deleteJournalEntry(title).rpc();
    }, 

    onSuccess: (signature) => {
      transactionToast(signature);
      accounts.refetch();
    },
  });


  return {
    accountQuery,
    updateEntry,
    deleteEntry,
  }
}
