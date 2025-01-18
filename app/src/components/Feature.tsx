import React from "react";
import { Text, View } from "react-native";

export function Feature({
  icon,
  children,
}: {
  icon: React.ElementType;
  children: React.ReactNode;
}) {
  return (
    <View className="flex flex-row w-full justify-between">
      <Text className="text-white text-lg">{children}</Text>
      {React.createElement(icon)}
    </View>
  );
}
