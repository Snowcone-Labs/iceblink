import { Text, View } from "react-native";
import { IceblinkLogo } from "./Logo";

export function IceblinkTextLogo({ ...props }) {
  return (
    <View className="flex flex-row items-center gap-2 p-5" {...props}>
      <IceblinkLogo />
      <Text className="text-white font-bold text-2xl"> Iceblink</Text>
    </View>
  );
}
