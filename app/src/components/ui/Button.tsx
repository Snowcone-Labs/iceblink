import React from "react";
import {
  StyleSheet,
  Text,
  TextStyle,
  TouchableOpacity,
  ViewStyle,
} from "react-native";

const colors = {
  danger: {
    bg: "#521111",
    fg: "#ff4f4f",
  },
  warning: {
    bg: "#6b450c",
    fg: "#ffbf42",
  },
  success: {
    bg: "#0c6b43",
    fg: "#20fea1",
  },
  primary: {
    bg: "#c8cbea",
    fg: "#2f314b",
  },
  secondary: {
    bg: "#1e202f",
    fg: "#f7f7fa",
  },
};

const shapes = {
  normal: {
    paddingHorizontal: 16,
    borderRadius: 10,
    height: 56,
    fontSize: 18,
  },
  normalNoPadding: {
    paddingHorizontal: 2,
    borderRadius: 10,
    height: 56,
    fontSize: 18,
  },
  square: {
    height: 56,
    width: 56,
    borderRadius: 10,
    fontSize: 18,
  },
  squareMedium: {
    height: 40,
    width: 40,
    borderRadius: 10,
    fontSize: 16,
  },
  squareSmall: {
    height: 32,
    width: 32,
    borderRadius: 8,
    fontSize: 16,
  },
  slim: {
    height: 40,
    borderRadius: 8,
    paddingHorizontal: 20,
    fontSize: 16,
  },
};

interface Props {
  children?: React.ReactNode;
  shape?: keyof typeof shapes;
  color?: keyof typeof colors;
  background?: string;
  foreground?: string;
  shadow?: boolean;
  disableRaiseOnFocus?: boolean;
  onPress?: () => void;
  style?: ViewStyle;
  textStyle?: TextStyle;
}

export default function Button({
  children,
  color = "primary",
  shape = "normal",
  background,
  foreground,
  shadow = false,
  disableRaiseOnFocus = false,
  onPress,
  style,
  textStyle,
}: Props) {
  const shapeStyle = shapes[shape];
  const buttonStyle: ViewStyle = {
    backgroundColor: background || colors[color].bg,
    ...(shadow && {
      shadowColor: "#000",
      shadowOffset: { width: 0, height: 2 },
      shadowOpacity: 0.2,
      shadowRadius: 2,
    }),
    ...shapeStyle,
    ...style,
  };

  const textStyles: TextStyle = {
    color: foreground || colors[color].fg,
    fontWeight: "bold",
    textAlign: "center",
    fontSize: shapeStyle.fontSize,
    ...textStyle,
  };

  return (
    <TouchableOpacity
      onPress={onPress}
      style={[
        styles.button,
        buttonStyle,
        disableRaiseOnFocus ? {} : styles.raiseOnFocus,
      ]}
      activeOpacity={0.8}
    >
      <Text style={textStyles}>{children}</Text>
    </TouchableOpacity>
  );
}

const styles = StyleSheet.create({
  button: {
    justifyContent: "center",
    alignItems: "center",
    borderWidth: 1,
    borderColor: "#fff2",
  },
  raiseOnFocus: {
    boxShadow:
      "0px 0px 0px 1px rgba(0, 0, 0, 0.25), var(--tw-ring-offset-shadow, 0 0 #0000), var(--tw-ring-shadow, 0 0 #0000), var(--tw-shadow)",
  },
});
