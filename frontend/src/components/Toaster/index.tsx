import { useToaster } from "src/hooks/useToaster";
import View from "./View";

export const Toaster = () => {
  const { message, visible, isError } = useToaster();

  return <View {...{ message, visible, isError }} />;
};
