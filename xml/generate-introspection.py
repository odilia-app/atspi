#!/usr/bin/env python3
#
# Takes DBus XML files and writes out a pair of introspection.[ch] files for inclusion
# in C code.

import argparse
import os
import sys
from xml.etree import ElementTree

CTEMPLATE = """
/*
 * This file has been auto-generated from the introspection data available
 * in the at-spi2-core repository. The D-Bus protocol is defined in this
 * repository, which can be found at:
 *
 * https://gitlab.gnome.org/GNOME/at-spi2-core
 *
 * DO NOT EDIT.
 */

%s
"""

HTEMPLATE = """
/*
 * This file has been auto-generated from the introspection data available
 * in the at-spi2-core repository. The D-Bus protocol is defined in this
 * repository, which can be found at:
 *
 * https://gitlab.gnome.org/GNOME/at-spi2-core
 *
 * DO NOT EDIT.
 */

#ifndef SPI_INTROSPECTION_DATA_H_
#define SPI_INTROSPECTION_DATA_H_

%s

#endif /* SPI_INTROSPECTION_DATA_H_ */
"""

DECTEMPLATE = """
extern const char *%s;
"""

DEFTEMPLATE = """
const char *%s =
%s;
"""

def convert_name (name):
    return "spi_" + name.replace (".", "_")

def convert_contents (contents):
    contents = contents.replace ("\"", "\\\"")
    literals = ["\"%s\"" % (line) for line in contents.split ("\n")]
    return "\n".join (literals)

def generate_introspection (inputs, c_output_filename, h_output_filename):
    #Open the output files.
    cfile = open (c_output_filename, "w")
    hfile = open (h_output_filename, "w")

    ccontents = ""
    hcontents = ""

    for input_filename in inputs:
        #Open the XML file and process includes.
        try:
            tree = ElementTree.parse (input_filename)
        except Exception as e:
            raise type(e)(f"Invalid XML while parsing {input_filename}: {str(e)}")

        root = tree.getroot ()

        for itf in root.findall ("interface"):
            #Get and convert the name of the interface.
            name = convert_name (itf.attrib["name"])

            contents = convert_contents (ElementTree.tostring (itf, encoding="unicode"))

            hcontents += DECTEMPLATE % (name)
            ccontents += DEFTEMPLATE % (name, contents)

    cfile.write (CTEMPLATE % (ccontents))
    hfile.write (HTEMPLATE % (hcontents))

    cfile.close ()
    hfile.close ()

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Create a C source file and header file from DBus XML files")
    parser.add_argument('sources', metavar='FILE.XML', nargs='+', help='DBus XML interface file')
    parser.add_argument('--c-output', metavar='OUT.C', required=True, help='Name out output C file')
    parser.add_argument('--h-output', metavar='OUT.H', required=True, help='Name out output H file')
    args = parser.parse_args()

    input_filename = sys.argv[1]
    c_output_filename = sys.argv[2]
    h_output_filename = sys.argv[3]

    generate_introspection (args.sources, args.c_output, args.h_output)
