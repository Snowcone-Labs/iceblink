import { IceblinkLogo } from "@/components/Logo";
import { Slot } from "expo-router";
import { Text, View } from "react-native";

export default function AuthLayout() {
  return (
    <View className="p-10 gap-10 h-full flex flex-col justify-center items-center">
      <View className="flex flex-row gap-4 items-center justify-center">
        <IceblinkLogo fancy={true} />
        <Text className="text-white text-center font-bold text-4xl">
          Iceblink
        </Text>
      </View>
      <Slot />
    </View>
  );
}
