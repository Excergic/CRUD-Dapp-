'use client'

import { useWallet } from '@solana/wallet-adapter-react'
import { WalletButton } from '../solana/solana-provider'
import { ExplorerLink } from '../cluster/cluster-ui'
import { useCounterProgram } from './counter-data-access'
import { CounterCreate, CounterList } from './counter-ui'
import { AppHero } from '../app-hero'
import { ellipsify } from '@/lib/utils'

export default function CounterFeature() {
  const { publicKey } = useWallet()
  const { programId } = useCounterProgram()

  return publicKey ? (
    <div>
      <AppHero
        title="CRUD-DAPP"
        subtitle={
          'This is a simple CRUD Dapp built with Anchor and React. It allows you to create, read, update, and delete entries on the Solana blockchain.'
        }
      >
        <p className="mb-6">
          <ExplorerLink path={`account/${programId}`} label={ellipsify(programId.toString())} />
        </p>
        <CounterCreate />
      </AppHero>
      <CounterList />
    </div>
  ) : (
    <div className="max-w-4xl mx-auto">
      <div className="hero py-[64px]">
        <div className="hero-content text-center">
          <WalletButton />
        </div>
      </div>
    </div>
  )
}
