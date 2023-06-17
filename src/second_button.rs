use pyo3::prelude::*;
use pyo3::types::PyTuple;

pub fn handle_second_button(file_path: &Option<String>) -> Option<(String, String)> {
    let result: Option<(String, String)> = Python::with_gil(|py| {
        // Load Python module
        let module = PyModule::import(py, "funcs").expect("No flying for you.");
        let load_json = module.getattr("load_json").unwrap();
        
        // Import functions from module
        let get_metadata = module.getattr("get_metadata").unwrap();
        let get_data = module.getattr("get_data").unwrap();
        //let compile_data = module.getattr("compile_data").unwrap();
        //let compile_metadata = module.getattr("compile_metadata").unwrap();
        //let save_data = module.getattr("save_data").unwrap();

        // Import the student_gender variable from Python
        let student_gender: PyObject = module.getattr("student_gender").unwrap().extract().unwrap();

        // Import the publication type variable from Python
        let publication_type: PyObject = module.getattr("publication_type_output").unwrap().extract().unwrap();

        // Import the study realism variable from Python
        let study_realism: PyObject = module.getattr("study_realism_output").unwrap().extract().unwrap();
        
        if let Some(file_path) = file_path {
            // load json data
            let data: PyObject = load_json.call1((file_path,)).unwrap().extract().unwrap();

            // get "eppiID" data
            let var1 = "ItemId"; // Replace with your desired variable
            let eppi_id: PyObject = get_metadata.call1((data.clone_ref(py), var1)).unwrap().extract().unwrap();

            // get "ShortTitle" data
            let var2 = "ShortTitle"; // Replace with your desired variable
            let ShortTitle: PyObject = get_metadata.call1((data.clone_ref(py), var2)).unwrap().extract().unwrap();

            // get "Year" data
            let var3 = "Year"; // Replace with your desired variable
            let Year: PyObject = get_metadata.call1((data.clone_ref(py), var3)).unwrap().extract().unwrap();

            // Call get_data function with student_gender data
            let gender: PyObject = get_data.call1((data.clone_ref(py), student_gender)).unwrap().extract().unwrap();

            // Call get_data function with student_gender data
            let pub_type: PyObject = get_data.call1((data.clone_ref(py), publication_type)).unwrap().extract().unwrap();

            // Call get_data function with student_gender data
            let study_real: PyObject = get_data.call1((data, study_realism)).unwrap().extract().unwrap();

            // Print data
            //println!("EppiID: {}", eppi_id);
            //println!("Gender: {}", gender);
            //println!("ShortTitle: {}", ShortTitle);
            //println!("Year: {}", Year);
            //println!("Pub: {}", pub_type);
            //println!("Study Realism: {}", study_real);

            // Return the gender value
            Some((gender.to_string(), eppi_id.to_string()))
        } else {
            None
        }
    });

    result
}
