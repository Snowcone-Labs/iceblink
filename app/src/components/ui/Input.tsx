import { MaterialIcons } from "@expo/vector-icons";
import React, { useEffect, useRef, useState } from "react";
import { StyleSheet, TextInput, TouchableOpacity, View } from "react-native";

interface Props {
  value: string;
  onChange: (value: string) => void;
  invalid?: boolean;
  disableRaiseOnFocus?: boolean;
  containerStyle?: object;
  showHideButton?: boolean;
  onEnter?: () => void;
  setRef?: (ref: React.RefObject<TextInput>) => void;
  blurOnEnter?: boolean;
  inputStyle?: object;
  placeholder?: string;
  secureTextEntry?: boolean;
}

export default function Input(props: Props) {
  const {
    value,
    onChange,
    invalid,
    disableRaiseOnFocus,
    containerStyle,
    showHideButton,
    onEnter,
    setRef,
    blurOnEnter,
    inputStyle,
    placeholder,
    secureTextEntry,
    ...remaining
  } = props;

  const inputRef = useRef<TextInput>(null);
  const [passwordVisible, setPasswordVisible] = useState(!secureTextEntry);

  useEffect(() => {
    if (setRef) {
      setRef(inputRef);
    }
  }, [inputRef]);

  const handleKeyPress = ({ nativeEvent }: { nativeEvent: any }) => {
    if (nativeEvent.key === "Enter") {
      if (onEnter) {
        onEnter();
      }
      if (blurOnEnter) {
        inputRef.current?.blur();
      }
    }
  };

  return (
    <View
      style={[
        styles.container,
        containerStyle,
        invalid && styles.invalidContainer,
      ]}
    >
      <TextInput
        ref={inputRef}
        style={[
          styles.input,
          inputStyle,
          disableRaiseOnFocus && styles.noFocusShadow,
        ]}
        placeholder={placeholder}
        secureTextEntry={!passwordVisible}
        onKeyPress={handleKeyPress}
        value={value} // Binds the value to the state
        onChangeText={onChange} // Updates the state when the input changes
        {...remaining}
      />
      {showHideButton && (
        <TouchableOpacity
          style={styles.iconButton}
          onPress={() => setPasswordVisible(!passwordVisible)}
        >
          <MaterialIcons
            name={passwordVisible ? "visibility-off" : "visibility"}
            size={24}
            color="#A0A3B1"
          />
        </TouchableOpacity>
      )}
    </View>
  );
}

const styles = StyleSheet.create({
  container: {
    position: "relative",
    width: "100%",
    borderWidth: 2,
    borderRadius: 8,
    borderColor: "#232538",
    backgroundColor: "#333",
    paddingHorizontal: 10,
    paddingVertical: 8,
  },
  invalidContainer: {
    borderColor: "red",
  },
  input: {
    fontSize: 16,
    color: "#FFF",
    width: "100%",
  },
  noFocusShadow: {
    shadowOpacity: 0,
  },
  iconButton: {
    position: "absolute",
    top: "50%",
    right: 10,
    transform: [{ translateY: -12 }],
  },
});
