import { PropsWithChildren } from "react";
import LoaderFallback from "src/components/Loader";
import ErrorFallback from "../ErrorFallback";

type QueryResult<T, E> = {
  data?: T;
  loading: boolean;
  error?: E;
};
interface PropsType<T, E> extends PropsWithChildren {
  query: QueryResult<T, E>;
}

const QueryWrapper = <T, E>({ query, children }: PropsType<T, E>) => {
  const { loading, data, error } = query;
  if (error) {
    console.error(error);
  }
  return (
    <>
      {loading && <LoaderFallback />}
      {data && children}
      {error && <ErrorFallback />}
    </>
  );
};

export default QueryWrapper;
