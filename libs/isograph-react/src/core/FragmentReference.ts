import { DataId } from './IsographEnvironment';
import { ReaderWithRefetchQueries } from '../core/entrypoint';
import { PromiseWrapper } from './PromiseWrapper';

// TODO type this better
export type VariableValue = string | number | boolean | null | object;

export type Variables = { readonly [index: string]: VariableValue };

export type ExtractData<
  T extends {
    data: object;
  },
> = T['data'];

export type ExtractParameters<
  T extends {
    parameters: object;
  },
> = T['parameters'];

export type FragmentReference<
  TReadFromStore extends { parameters: object; data: object },
  TClientFieldValue,
> = {
  readonly kind: 'FragmentReference';
  readonly readerWithRefetchQueries: PromiseWrapper<
    ReaderWithRefetchQueries<TReadFromStore, TClientFieldValue>
  >;
  readonly root: DataId;
  readonly variables: ExtractParameters<TReadFromStore>;
  readonly networkRequest: PromiseWrapper<void, any>;
};

export function stableIdForFragmentReference(
  fragmentReference: FragmentReference<any, any>,
): string {
  return `${fragmentReference.root}/TODO_FRAGMENT_NAME/${serializeVariables(fragmentReference.variables ?? {})}`;
}

function serializeVariables(variables: Variables) {
  let s = '';
  const keys = Object.keys(variables);
  keys.sort();
  for (const key of keys) {
    s += `${key}:${variables[key]},`;
  }
  return s;
}
