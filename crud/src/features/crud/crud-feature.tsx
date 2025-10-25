import { useSolana } from '@/components/solana/use-solana'
import { WalletDropdown } from '@/components/wallet-dropdown'
import { AppHero } from '@/components/app-hero'
import { CrudUiProgramExplorerLink } from './ui/crud-ui-program-explorer-link'
import { CrudUiCreate } from './ui/crud-ui-create'
import { CrudUiProgram } from '@/features/crud/ui/crud-ui-program'

export default function CrudFeature() {
  const { account } = useSolana()

  if (!account) {
    return (
      <div className="max-w-4xl mx-auto">
        <div className="hero py-[64px]">
          <div className="hero-content text-center">
            <WalletDropdown />
          </div>
        </div>
      </div>
    )
  }

  return (
    <div>
      <AppHero title="Crud" subtitle={'Run the program by clicking the "Run program" button.'}>
        <p className="mb-6">
          <CrudUiProgramExplorerLink />
        </p>
        <CrudUiCreate account={account} />
      </AppHero>
      <CrudUiProgram />
    </div>
  )
}
