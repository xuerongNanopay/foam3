/**
 * @license
 * Copyright 2017 The FOAM Authors. All Rights Reserved.
 * http://www.apache.org/licenses/LICENSE-2.0
 */

package foam.core;

import foam.nanos.logger.Logger;
import java.io.File;
import java.io.FileReader;
import java.io.IOException;
import java.io.StringWriter;
import java.util.ArrayList;
import java.util.Iterator;
import java.util.List;
import javax.xml.XMLConstants;
import javax.xml.parsers.DocumentBuilder;
import javax.xml.parsers.DocumentBuilderFactory;
import javax.xml.parsers.ParserConfigurationException;
import javax.xml.stream.*;
import javax.xml.stream.XMLInputFactory;
import javax.xml.stream.XMLStreamException;
import javax.xml.transform.*;
import javax.xml.transform.dom.DOMSource;
import javax.xml.transform.stream.StreamResult;
import org.w3c.dom.Document;
import org.w3c.dom.Element;

public class XMLSupport {

  public static List<FObject> fromXML(X x, XMLStreamReader xmlr) {
    return fromXML(x, xmlr, null);
  }

  public static List<FObject> fromXML(X x, XMLStreamReader xmlr, Class defaultClass) {
    List<FObject> objList = new ArrayList<FObject>();
    try {
      int eventType;
      while ( xmlr.hasNext() ) {
        eventType = xmlr.next();
        switch ( eventType ) {
          case XMLStreamConstants.START_ELEMENT:
            var elName = xmlr.getLocalName();
            var objClass = defaultClass != null && elName.equals(defaultClass.getSimpleName()) ? defaultClass : null;

            var obj = createObj(x, xmlr, objClass);
            if ( obj != null ) {
              objList.add(obj);
            }
            break;
        }
      }
      xmlr.close();
    } catch (XMLStreamException ex) {
      Logger logger = (Logger) x.get("logger");
      logger.error("Could not read from file with existing XMLStreamReader");
    }
    return objList;
  }

  public static FObject createObj(X x, XMLStreamReader xmlr) {
    return createObj(x, xmlr, null);
  }

  public static FObject createObj(X x, XMLStreamReader xmlr, Class defaultClass) {
    Object clsInstance = null;
    String objClass;
    try {
      //objClass = xmlr.getAttributeValue(null, "class");

      if ( defaultClass == null ) {
        objClass = xmlr.getAttributeValue(null, "class");
        Class cls = Class.forName(objClass);
        clsInstance = x.create(cls);

        //x.create(defaultClass);
      } else {
        //x.create(Class.forName(objClass));
        clsInstance = x.create(defaultClass);
      }

      copyFromXML(x, (FObject) clsInstance, xmlr);
    } catch (XMLStreamException ex ) {
      Logger logger = (Logger) x.get("logger");
      logger.error("Error while reading file");
    } catch (Throwable t) {
      Logger logger = (Logger) x.get("logger");
      logger.error("XML parsing error", t);
    }
    return (FObject) clsInstance;
  }

  public static List<FObject> fromXML(X x, String fileName) throws IOException {
    XMLInputFactory xmlInputFactory = XMLInputFactory.newInstance();
    xmlInputFactory.setProperty(XMLInputFactory.SUPPORT_DTD, false);
    XMLStreamReader xmlr = null;
    try {
      xmlr = xmlInputFactory.createXMLStreamReader(new FileReader(fileName));
    } catch (IOException ex) {
      Logger logger = (Logger) x.get("logger");
      logger.error("Could not create/file with given fileName");
    } catch (XMLStreamException ex) {
      Logger logger = (Logger) x.get("logger");
      logger.error("Error reading file: ", fileName);
    }
    return fromXML(x, xmlr);
  }

  public static void copyFromXML(X x, FObject obj, XMLStreamReader reader) throws XMLStreamException {
    try {
      var cInfo = obj.getClassInfo();
      var objElName = reader.getLocalName();
      while ( reader.hasNext() ) {
        int eventType;
        eventType = reader.next();
        switch ( eventType ) {
          case XMLStreamConstants.START_ELEMENT:
            var prop = (PropertyInfo) cInfo.getAxiomByNameOrShortName(reader.getLocalName());
            if ( prop != null ) {
              prop.copyFromXML(x, obj, reader);
              prop = null;
            }
            break;
          case XMLStreamConstants.END_ELEMENT:
            if ( reader.getLocalName().equals(objElName) ) {
              return;
            }
            break;
        }
      }
    } catch (XMLStreamException ex) {
      Logger logger = (Logger) x.get("logger");
      logger.error("Premature end of xml file");
    }
  }

  public static Document createDoc() {
    Document doc = null;
    try {
      DocumentBuilderFactory dbFactory = DocumentBuilderFactory.newInstance();
      dbFactory.setFeature(XMLConstants.FEATURE_SECURE_PROCESSING, true);
      DocumentBuilder dBuilder = dbFactory.newDocumentBuilder();
      doc = dBuilder.newDocument();
    } catch (ParserConfigurationException ex) {
    }
    return doc;
  }

  public static Transformer createTransformer() {
    Transformer transformer = null;
    try {
      TransformerFactory tf = TransformerFactory.newInstance();
      tf.setAttribute(XMLConstants.ACCESS_EXTERNAL_DTD, "");
      transformer = tf.newTransformer();
      transformer.setOutputProperty(OutputKeys.INDENT, "yes");
      transformer.setOutputProperty("{http://xml.apache.org/xslt}indent-amount", "2");
    } catch (TransformerConfigurationException ex) {
    }
    return transformer;
  }
}
