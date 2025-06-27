import { p as AuthContext, s as checkPassword } from '../../shared/better-auth.Ct9ORPbF.mjs';
import '../../shared/better-auth.Bi8FQwDD.mjs';
import 'zod';
import '../../shared/better-auth.COrZG5RM.mjs';
import 'jose';
import 'kysely';
import 'better-call';
import 'better-sqlite3';
import 'bun:sqlite';

interface HaveIBeenPwnedOptions {
    customPasswordCompromisedMessage?: string;
}
declare const haveIBeenPwned: (options?: HaveIBeenPwnedOptions) => {
    id: "haveIBeenPwned";
    init(ctx: AuthContext): {
        context: {
            password: {
                hash(password: string): Promise<string>;
                verify: (data: {
                    password: string;
                    hash: string;
                }) => Promise<boolean>;
                config: {
                    minPasswordLength: number;
                    maxPasswordLength: number;
                };
                checkPassword: typeof checkPassword;
            };
        };
    };
    $ERROR_CODES: {
        readonly PASSWORD_COMPROMISED: "The password you entered has been compromised. Please choose a different password.";
    };
};

export { haveIBeenPwned };
export type { HaveIBeenPwnedOptions };
