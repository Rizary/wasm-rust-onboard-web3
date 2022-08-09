import type { SelectAccountOptions, Account } from './types';
declare const accountSelect: (options: SelectAccountOptions) => Promise<Account[]>;
export default accountSelect;
