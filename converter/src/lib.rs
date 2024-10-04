use quick_xml::events::{BytesEnd, BytesStart, BytesText, Event};
use quick_xml::Writer;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter};

pub fn convert(input_path: &str, output_path: &str, delimiter: char) -> Result<(), Box<dyn Error>> {
    // Open the input file
    let input_file = File::open(input_path)?;
    let reader = BufReader::new(input_file);

    // Open the output XML file
    let output_file = File::create(output_path)?;
    let writer = BufWriter::new(output_file);
    let mut xml_writer = Writer::new_with_indent(writer, b' ', 4);

    // Write the XML declaration
    let decl = quick_xml::events::BytesDecl::new("1.0", Some("UTF-8"), None);
    xml_writer.write_event(Event::Decl(decl))?;

    // Start the root <Document> element
    let doc_start = BytesStart::new("Document");
    xml_writer.write_event(Event::Start(doc_start))?;

    let mut headers: Vec<String> = Vec::new();

    for line_result in reader.lines() {
        let line = line_result?;
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue; // Skip empty lines
        }

        let fields: Vec<&str> = trimmed.split(delimiter).collect();

        if headers.is_empty() {
            // This is the header line
            headers = fields.iter().map(|s| s.to_string()).collect();
            continue;
        }

        // Start a new <Row>
        let row_start = BytesStart::new("Row");
        xml_writer.write_event(Event::Start(row_start))?;

        for (i, field) in fields.iter().enumerate() {
            // Start <Field>
            let field_start = BytesStart::new("Field");
            xml_writer.write_event(Event::Start(field_start))?;

            // <FieldName pos="x">Header</FieldName>
            let mut field_name_start = BytesStart::new("FieldName");
            let pos_str = (i + 1).to_string();
            field_name_start.push_attribute(("pos", pos_str.as_str()));
            xml_writer.write_event(Event::Start(field_name_start))?;
            let field_name_text = BytesText::new(&headers[i]);
            xml_writer.write_event(Event::Text(field_name_text))?;
            xml_writer.write_event(Event::End(BytesEnd::new("FieldName")))?;

            // <FieldContent pos="x">Content</FieldContent>
            let mut field_content_start = BytesStart::new("FieldContent");
            let pos_str_content = (i + 1).to_string();
            field_content_start.push_attribute(("pos", pos_str_content.as_str()));
            xml_writer.write_event(Event::Start(field_content_start))?;
            let field_content_text = BytesText::new(field);
            xml_writer.write_event(Event::Text(field_content_text))?;
            xml_writer.write_event(Event::End(BytesEnd::new("FieldContent")))?;

            // End <Field>
            xml_writer.write_event(Event::End(BytesEnd::new("Field")))?;
        }

        // Add <Source nametype="leafnoext">empty_lines</Source>
        let mut source_start = BytesStart::new("Source");
        source_start.push_attribute(("nametype", "leafnoext"));
        xml_writer.write_event(Event::Start(source_start))?;
        let source_text = BytesText::new("empty_lines");
        xml_writer.write_event(Event::Text(source_text))?;
        xml_writer.write_event(Event::End(BytesEnd::new("Source")))?;

        // Add <Worksheet>empty_lines</Worksheet>
        let worksheet_start = BytesStart::new("Worksheet");
        xml_writer.write_event(Event::Start(worksheet_start))?;
        let worksheet_text = BytesText::new("empty_lines");
        xml_writer.write_event(Event::Text(worksheet_text))?;
        xml_writer.write_event(Event::End(BytesEnd::new("Worksheet")))?;

        // End <Row>
        xml_writer.write_event(Event::End(BytesEnd::new("Row")))?;
    }

    // Close the root <Document> element
    xml_writer.write_event(Event::End(BytesEnd::new("Document")))?;

    println!("Conversion completed successfully.");

    Ok(())
}
