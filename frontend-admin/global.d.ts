type Result<T, E> =
  | { value: T; error?: never }
  | { error: E; value?: never };

type Option<T> =
  | { some: true; value: T }
  | { some: false };

type Err = {
  message: string;
};

declare function match<T, E>(
    input: Result<T, E>,
    cases: {
      Ok: (value: T) => T;
      Err: (error: E) => E;
    }
  ): R;
  
declare function match<T, R>(
input: Option<T>,
cases: {
    Some: (value: T) => R;
    None?: () => R;
},
defaultCase: () => R
): R;

declare function match<T, R>(
input: T,
cases: {
    [key: string]: (value: T) => R;
},
defaultCase: () => R
): R;