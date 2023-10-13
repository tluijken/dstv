# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

You may also find the [Upgrade Guide](https://rust-random.github.io/book/update.html) useful.

## [0.4.0] - Released Oct 13th 2023
### Added
* More unit tests to secure contour representations
### Fixed
* Added better support for rounded corners and negative radius values on the
contours.

## [0.3.0] - Released Jul 31th 2023
### Fixed
* Slot VSG representation. Was previously equal to other 'Hole' representations,
  but now has a true slot representation.

## [0.2.0] - Released Jul 11th 2023
### Added
* Representation for Bezels
### Fixed
* Contour rendering issues

## [0.1.0] - Released June 2nd 2023
### Added
* Initial parsing
* SVG rendering functionality
