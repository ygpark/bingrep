BOOKNAME=USER_MANUAL
CHROME="/c/Program Files/Google/Chrome/Application/chrome.exe"
ACRORD="/c/Program Files (x86)/Adobe/Acrobat Reader DC/Reader/AcroRd32.exe"
ROOT="`pwd`"

all: html pdf

html:
	asciidoctor $(BOOKNAME).adoc

htmlrun: html
	-if [ -e $(ROOT)/$(BOOKNAME).html ] ; then $(CHROME) $(ROOT)/$(BOOKNAME).html ; fi

pdf:
	asciidoctor-pdf -a scripts=cjk -a pdf-fontsdir=./fonts -a pdf-theme=./korean-theme.yml $(BOOKNAME).adoc

pdfrun: pdf
	-if [ -e $(ROOT)/$(BOOKNAME).pdf ] ; then $(ACRORD) $(ROOT)/$(BOOKNAME).pdf ; fi

test:
	asciidoctor TEST.adoc

readme:
	asciidoctor README.adoc

clean:
	rm -f $(BOOKNAME).pdf
	rm -f $(BOOKNAME).html
	rm -f README.pdf
	rm -f README.html

font:
	mkdir -p fonts
	install-font.bat

bundle:
	#gem install rake
	gem install asciidoctor
	gem install asciidoctor-pdf
	gem install asciidoctor-pdf-cjk
	gem install asciidoctor-pdf-cjk-kai_gen_gothic
	gem install json
	gem install awesome_print
	gem install coderay
	gem install pygments.rb
	gem install thread_safe
	gem install epubcheck-ruby
	gem install html-proofer
	