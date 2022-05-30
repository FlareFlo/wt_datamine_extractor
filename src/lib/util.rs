pub fn parameter_to_data(file: &str, parameter: &str) -> Option<String> {
	file.find(&format!("\"{}\"", parameter)).map(|value| {
		let position_value = file.split_at(value + parameter.len() + 3).1;
		let cropped_value = position_value.split_once('\n').unwrap().0;
		let cleaned_value = cropped_value.replace(',', "").trim().to_owned(); // Sub-objects somehow contain a comma
		cleaned_value
	})
}

#[cfg(test)]
mod tests {

}